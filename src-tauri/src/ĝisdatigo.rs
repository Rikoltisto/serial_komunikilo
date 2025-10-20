use serde::Serialize;
use std::{sync::Mutex};
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

pub struct PritraktaĜisdatigo(pub Mutex<Option<Update>>);

#[tauri::command]
pub async fn kontroli_ĝisdatigojn(
    aplikaĵo: tauri::AppHandle,
    pritrakta_ĝisdatigo: State<'_, PritraktaĜisdatigo>,
) -> Rezulto<Option<ĜisdatigiMetadatumon>> {
    //Akiri Ĝisdatigojn.
    let ĝisdatigi = aplikaĵo.updater()?.check().await?;
    //Generi Sisteman Ĝisdatigan Datumaron.
    let ĝisdatigi_metadatumon = ĝisdatigi.as_ref().map(|ĝ|
        ĜisdatigiMetadatumon {
            versio: ĝ.version.clone(),
            nuna_versio: ĝ.current_version.clone(),
            notoj: ĝ.body.clone().unwrap(),
            dato: ĝ.date.map(|d| d.to_string()).unwrap(),
        }
    );
    //Enmeti Ĝisdatigon en la Statan Administradon.
    *pritrakta_ĝisdatigo.0.lock().unwrap() = ĝisdatigi;
    //Redoni Sisteman Ĝisdatigon.
    Ok(ĝisdatigi_metadatumon)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn elŝuti_kaj_ĝisdatigi(
    pritrakta_ĝisdatigo: State<'_, PritraktaĜisdatigo>,
    pri_evento: Channel<ElŝutaEvento>,
) -> Rezulto<()> {
    //Kontroli Ĉu Ĝisdatigo Ekzistas.
    let Some(ĝisdatigi) = pritrakta_ĝisdatigo.0.lock().unwrap().take() else {
        return Err(Eraro::NeniuPritraktaĜisdatigo);
    };

    let mut komencita = false;

    ĝisdatigi
        .download_and_install(
            |ĉunk_longo, enhava_longo| {
                //Sendi Unufoje Elŝutkomencon.
                if !komencita {
                    pri_evento.send(ElŝutaEvento::Komencita { enhava_longo }).unwrap();
                    komencita = true;
                }

                pri_evento.send(ElŝutaEvento::Progreso { ĉunk_longo }).unwrap();
            },
            || {
                pri_evento.send(ElŝutaEvento::Finita).unwrap();
            }
        )
        .await?;

    Ok(())
}
