use std::sync::Mutex;

use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod komandoj;
#[path = "ĝisdatigo.rs"]
mod ĝisdatigo;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|aplikaĵo| {
            let fenestro = aplikaĵo.get_webview_window("ĉefa").unwrap();
            fenestro.set_maximizable(false).unwrap();
            fenestro.set_resizable(false).unwrap();
            Ok(())
        })
        .manage(ĝisdatigo::PritraktaĜisdatigo(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            ĝisdatigo::kontroli_ĝisdatigojn,
            ĝisdatigo::elŝuti,
            ĝisdatigo::instali,
            komandoj::eliri,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
