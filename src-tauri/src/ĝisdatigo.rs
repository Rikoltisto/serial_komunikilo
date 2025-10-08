use tauri_plugin_updater::{Update, UpdaterExt};


//Kontroli Ĉu Estas Ĝisdatigoj Kaj Elŝuti Kaj Instali Ilin.
// pub async fn ĝisdatigi(aplikaĵo: tauri::AppHandle, ĝisdatigi: Update) -> tauri_plugin_updater::Result<()> {
//     let mut estas_elŝutita = 0;
    
//     ĝisdatigi
//         .download_and_install(
//         |ĉunk_longo, enhava_longo | {
//             estas_elŝutita += ĉunk_longo;
//         }, 
//         || {

//         },
//         )
//         .await?;
    
//     aplikaĵo.restart();

//     Ok(())
// }

pub async fn kontroli_ĝisdatigojn(aplikaĵo: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(ĝisdatigi) = aplikaĵo.updater()?.check().await? {
        
    }

    Ok(())
}
