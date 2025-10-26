#[tauri::command]
pub fn eliri(aplikaĵo: tauri::AppHandle) {
    aplikaĵo.exit(0);
}
