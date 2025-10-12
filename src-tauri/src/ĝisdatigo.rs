use tauri_plugin_updater::{Update, UpdaterExt};
use serde::Serialize;

#[derive(Serialize)]
pub struct VersiaInformo {
    versio: String,
    nuna_versio: String,
    notoj: String,
    dato: String,
}

#[tauri::command]
pub async fn kontroli_ĝisdatigojn(aplikaĵo: tauri::AppHandle) -> tauri_plugin_updater::Result<(bool, Option<VersiaInformo>)> {
    if let Some(ĝisdatigi) = aplikaĵo.updater()?.check().await? {
        return Ok((true, Some(akiri_versian_informon(ĝisdatigi).await)));
    }
    Ok((false, None))
}

#[tauri::command]
pub async fn elŝuti_kaj_ĝisdatigi(aplikaĵo: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    let mut estas_elŝutita = 0;

    aplikaĵo
        .updater()?
        .check()
        .await?
        .unwrap()
        .download_and_install(|ĉunk_longo, enhava_longo| {
            estas_elŝutita += ĉunk_longo;
        },|| {})
        .await?;

    aplikaĵo.restart();
}

async fn akiri_versian_informon(ĝisdatigi: Update) -> VersiaInformo {
    VersiaInformo {
        versio: ĝisdatigi.version,
        nuna_versio: ĝisdatigi.current_version,
        notoj: ĝisdatigi.body.unwrap(),
        dato: ĝisdatigi.date.map(|d| d.to_string()).unwrap(),
    }
}
