use serde::Serialize;
use std::{
    env,
    fs::{self, File},
    io::Write,
    process::Command,
    sync::Mutex,
};
use tauri::{ipc::Channel, State};
use tauri_plugin_updater::{Update, UpdaterExt};

//Result Alias.
type Result<T> = std::result::Result<T, Error>;

//Error Enum.
#[derive(Debug, thiserror::Error, Serialize)]
pub enum Error {
    #[error(transparent)]
    Updater(#[from] tauri_plugin_updater::Error),
    #[error("there is no pending update")]
    NoPendingUpdate,
}

#[derive(Serialize)]
#[serde(tag = "event", content = "data")]
pub enum DownloadEvent {
    Started { content_length: Option<u64> },
    Progress { downloaded: usize },
    Finished,
}

#[derive(Serialize)]
pub struct UpdateMetadata {
    version: String,
    current_version: String,
    note: String,
    date: String,
}

pub struct PendingUpdate(pub Mutex<Option<Update>>);

#[tauri::command]
pub async fn check_for_updates(
    app: tauri::AppHandle,
    pending_update: State<'_, PendingUpdate>,
) -> Result<Option<UpdateMetadata>> {
    //Updater.
    let updater = app.updater()?.check().await?;
    //Generate system update metadata.
    let update_metadata = updater.as_ref().map(|updater| UpdateMetadata {
        version: updater.version.clone(),
        current_version: updater.current_version.clone(),
        note: updater.body.clone().unwrap(),
        date: updater.date.map(|d| d.to_string()).unwrap(),
    });
    //Store the updater in the SoftwareUpdater struct.
    *pending_update.0.lock().unwrap() = updater;
    //Return the system update metadata.
    Ok(update_metadata)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn download(
    pending_update: State<'_, PendingUpdate>,
    on_events: Channel<DownloadEvent>,
) -> Result<String> {
    //Check for Updates.
    let Some(updater) = pending_update.0.lock().unwrap().take() else {
        return Err(Error::NoPendingUpdate);
    };

    let mut started  = false;
    let mut downloaded = 0;

    let file_data = updater
        .download(
            |chunk_length, content_length| {
                //Trigger a download-start event.
                if !started  {
                    on_events
                        .send(DownloadEvent::Started { content_length })
                        .unwrap();
                    started  = true;
                }

                downloaded += chunk_length;
                on_events
                    .send(DownloadEvent::Progress { downloaded })
                    .unwrap();
            },
            || {
                on_events.send(DownloadEvent::Finished).unwrap();
            },
        )
        .await?;
    //Get the current path of the EXE file.
    let current_executable = env::current_exe();
    //Get the program's installation folder and create a temporary directory named "Temp".
    let application_folder = current_executable.unwrap().parent().unwrap().join("temp");
    //Create Folder.
    fs::create_dir_all(&application_folder).unwrap();
    //Create the full path.
    let file_path = &application_folder.join(
        updater
            .download_url
            .path_segments()
            .and_then(|segment| segment.last())
            .unwrap(),
    );
    //Save File.
    let mut file = File::create(file_path).unwrap();
    file.write_all(&file_data).unwrap();
    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn install(app: tauri::AppHandle, file_path: String) -> Result<()> {
    //Get the current path of the EXE file.
    let current_executable = env::current_exe().unwrap();

    Command::new("powershell")
        .args(&[
            "-Command",
            &format!(
                "Write-Host 'Begin Installation...'; \
            5..1 | % {{ Write-Host \"$_\"; Start-Sleep -Seconds 1 }}; \
            Start-Process '{}' -ArgumentList '/S','/quiet' -Wait; \
            Write-Host 'Installation finished. Removing the installer...'; \
            Remove-Item '{}'; \
            Write-Host 'The installer has been removed. Launching the new version...'; \
            3..1 | % {{ Write-Host \"$_\"; Start-Sleep -Seconds 1 }}; \
            Start-Process '{}'; \
            Write-Host 'All tasks completed...'
            Start-Sleep -Seconds 2; \
            exit",
                file_path,
                file_path,
                current_executable.to_string_lossy().to_string(),
            ),
        ])
        .spawn()
        .unwrap();

    app.exit(0);
    Ok(())
}
