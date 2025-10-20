#[tauri::command]
pub fn eliri(aplikaĵo: tauri::AppHandle) {
    aplikaĵo.exit(0);
}

#[tauri::command]
pub fn restartigi(aplikaĵo: tauri::AppHandle) {
    aplikaĵo.restart();
}
