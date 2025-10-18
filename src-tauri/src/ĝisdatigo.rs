use serde::Serialize;
use std::sync::Mutex;
use tauri::State;
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

#[derive(Clone, Serialize)]
#[serde(tag = "evento", content = "datumo")]
pub enum ElŝutaEvento {
    Komencita { enhava_longo: Option<u64> },
    Progreso { ĉunk_longo: usize },
    Finita,
}

#[derive(Serialize)]
pub struct ĜisdatigiMetadatumon {
    versio: String,
    nuna_versio: String,
    notoj: String,
    dato: String,
}

pub struct PritraktaĜisdatigo(Mutex<Option<Update>>);

#[tauri::command]
pub async fn kontroli_ĝisdatigojn(
    aplikaĵo: tauri::AppHandle,
    pritrakta_ĝisdatigo: State<'_, PritraktaĜisdatigo>,
) -> Rezulto<Option<ĜisdatigiMetadatumon>> {
    *pritrakta_ĝisdatigo.0.lock().unwrap()  = aplikaĵo.updater()?.check().await?;

    Ok(None)
}

#[tauri::command]
pub async fn elŝuti_kaj_ĝisdatigi(
    aplikaĵo: tauri::AppHandle,
) -> tauri_plugin_updater::Result<()> {
    let mut estas_elŝutita = 0;

    aplikaĵo
        .updater()?
        .check()
        .await?
        .unwrap()
        .download_and_install(
            |ĉunk_longo, enhava_longo| {
                estas_elŝutita += ĉunk_longo;
            },
            || {},
        )
        .await?;

    aplikaĵo.restart();
}

async fn akiri_versian_informon(ĝisdatigi: Update) -> ĜisdatigiMetadatumon {
    ĜisdatigiMetadatumon {
        versio: ĝisdatigi.version,
        nuna_versio: ĝisdatigi.current_version,
        notoj: ĝisdatigi.body.unwrap(),
        dato: ĝisdatigi.date.map(|d| d.to_string()).unwrap(),
    }
}
