use std::sync::Mutex;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[path ="ĝisdatigo.rs"]
mod ĝisdatigo;
mod komandoj;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
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
