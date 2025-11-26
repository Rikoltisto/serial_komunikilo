#[tauri::command]
pub fn exit(app: tauri::AppHandle) {
    app.exit(0);
}
