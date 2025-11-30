use std::sync::Mutex;

use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod serial;
mod updater;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_maximizable(false).unwrap();
            window.set_resizable(false).unwrap();
            Ok(())
        })
        .manage(updater::PendingUpdate(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            updater::check_for_updates,
            updater::download,
            updater::install,
            commands::exit,
            serial::get_all_serial_port,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
