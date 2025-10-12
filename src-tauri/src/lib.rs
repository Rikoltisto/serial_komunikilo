// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[path ="ĝisdatigo.rs"]
mod ĝisdatigo;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            ĝisdatigo::kontroli_ĝisdatigojn,
            ĝisdatigo::elŝuti_kaj_ĝisdatigi,
            eliri
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn eliri(aplikaĵo: tauri::AppHandle) {
    aplikaĵo.exit(0);
}
