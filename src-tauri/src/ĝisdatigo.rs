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

//Redoni Rezultan Alinomon.
type Rezulto<T> = std::result::Result<T, Eraro>;

//Erara Enum
#[derive(Debug, thiserror::Error, Serialize)]
pub enum Eraro {
    #[error(transparent)]
    Ĝisdatigilo(#[from] tauri_plugin_updater::Error),
    #[error("there is no pending update")]
    NeniuPritraktaĜisdatigo,
}

#[derive(Serialize)]
#[serde(tag = "evento", content = "datumo")]
pub enum ElŝutaEvento {
    Komencita { enhava_longo: Option<u64> },
    Progreso { elŝutita: usize },
    Finita,
}

#[derive(Serialize)]
pub struct ĜisdatigiMetadatumon {
    versio: String,
    nuna_versio: String,
    notoj: String,
    dato: String,
}

pub struct PritraktaĜisdatigo(pub Mutex<Option<Update>>);

#[tauri::command]
pub async fn kontroli_ĝisdatigojn(
    aplikaĵo: tauri::AppHandle,
    pritrakta_ĝisdatigo: State<'_, PritraktaĜisdatigo>,
) -> Rezulto<Option<ĜisdatigiMetadatumon>> {
    //Akiri Ĝisdatigojn.
    let ĝisdatigi = aplikaĵo.updater()?.check().await?;
    //Generi Sisteman Ĝisdatigan Datumaron.
    let ĝisdatigi_metadatumon = ĝisdatigi.as_ref().map(|ĝ| ĜisdatigiMetadatumon {
        versio: ĝ.version.clone(),
        nuna_versio: ĝ.current_version.clone(),
        notoj: ĝ.body.clone().unwrap(),
        dato: ĝ.date.map(|d| d.to_string()).unwrap(),
    });
    //Enmeti Ĝisdatigon en la Statan Administradon.
    *pritrakta_ĝisdatigo.0.lock().unwrap() = ĝisdatigi;
    //Redoni Sisteman Ĝisdatigon.
    Ok(ĝisdatigi_metadatumon)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn elŝuti(
    pritrakta_ĝisdatigo: State<'_, PritraktaĜisdatigo>,
    pri_evento: Channel<ElŝutaEvento>,
) -> Rezulto<String> {
    //Kontroli Ĉu Ĝisdatigo Ekzistas.
    let Some(ĝisdatigi) = pritrakta_ĝisdatigo.0.lock().unwrap().take() else {
        return Err(Eraro::NeniuPritraktaĜisdatigo);
    };

    let mut komencita = false;
    let mut elŝutita = 0;

    let dosieraj_datumoj = ĝisdatigi
        .download(
            |ĉunk_longo, enhava_longo| {
                //Sendi Unufoje Elŝutkomencon.
                if !komencita {
                    pri_evento
                        .send(ElŝutaEvento::Komencita { enhava_longo })
                        .unwrap();
                    komencita = true;
                }

                elŝutita += ĉunk_longo;
                pri_evento
                    .send(ElŝutaEvento::Progreso { elŝutita })
                    .unwrap();
            },
            || {
                pri_evento.send(ElŝutaEvento::Finita).unwrap();
            },
        )
        .await?;
    //Akiri la nunan lokon de la exe-dosiero.
    let nuna_plenumebla = env::current_exe();
    //Akiri la instaladan dosierujon de la programo kaj krei provizoran dosierujon 'provizora'.
    let aplikaĵa_dosierujo = nuna_plenumebla.unwrap().parent().unwrap().join("provizora");
    //Krei dosierujon.
    fs::create_dir_all(&aplikaĵa_dosierujo).unwrap();
    //Krei la plenan vojon.
    let dosiera_vojo = &aplikaĵa_dosierujo.join(
        ĝisdatigi
            .download_url
            .path_segments()
            .and_then(|segmentoj| segmentoj.last())
            .unwrap(),
    );
    //Konservi dosieron.
    let mut dosiero = File::create(dosiera_vojo).unwrap();
    dosiero.write_all(&dosieraj_datumoj).unwrap();
    Ok(dosiera_vojo.to_string_lossy().to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn instali(
    aplikaĵo: tauri::AppHandle,
    dosiera_vojo: String
) -> Rezulto<()> {
    //Akiri la nunan lokon de la exe-dosiero.
    let nuna_plenumebla = env::current_exe().unwrap();

    Command::new("powershell")
        .args(&["-Command", &format!(
            "Write-Host 'Komenci instaladon...'; \
            5..1 | % {{ Write-Host \"`$_...\"; Start-Sleep -Seconds 1 }}; \
            Start-Process '{}' -ArgumentList '/S','/quiet' -Wait; \
            Write-Host 'Instalado kompletigita, forigu la instalpakaĵon...'; \
            Remove-Item '{}'; \
            Write-Host 'Instalpakaĵo estas forigita, startigu la novan version...'; \
            3..1 | % {{ Write-Host \"`$_...\"; Start-Sleep -Seconds 1 }}; \
            Start-Process '{}'; \
            Write-Host '所有操作完成'",
            dosiera_vojo,
            dosiera_vojo,
            nuna_plenumebla.to_string_lossy().to_string(),
        )])
        .spawn()
        .unwrap();

    //Forigi la instalan dosieron.
    aplikaĵo.restart();
}
