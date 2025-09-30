// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use tauri_plugin_updater::UpdaterExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|aplikaĵo| {
            let anstataŭigilo = aplikaĵo.handle().clone();

            tauri::async_runtime::spawn(async move {
                ĝisdatigi(anstataŭigilo).await.unwrap();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![akiri_disponeblajn_seriaportojn])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn akiri_disponeblajn_seriaportojn() {
    println!("akiri_disponeblajn_seriaportojn");
}

//Kontroli Ĉu Estas Ĝisdatigoj Kaj Elŝuti Kaj Instali Ilin.
async fn ĝisdatigi(aplikaĵo: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(ĝisdatigi) = aplikaĵo.updater()?.check().await? {
        let mut estas_elŝutita = 0;
        
        ĝisdatigi
            .download_and_install(
            |ĉunk_longo, enhava_longo | {
                estas_elŝutita += ĉunk_longo;
                println!("estas_elŝutita {estas_elŝutita} de {enhava_longo:?}");
            }, 
            || {
                println!("Elŝuto finis.");
            },
            )
            .await?;
        
        println!("Ĝisdatigo instalita.");
        aplikaĵo.restart();
    }

    Ok(())
}
