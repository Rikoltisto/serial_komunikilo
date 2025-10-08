// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[path ="ĝisdatigo.rs"]
mod ĝisdatigo;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|aplikaĵo| {
            let anstataŭigilo = aplikaĵo.handle().clone();
            
            tauri::async_runtime::spawn(async move {
                ĝisdatigo::kontroli_ĝisdatigojn(anstataŭigilo);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![akiri_disponeblajn_seriaportojn])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn akiri_disponeblajn_seriaportojn() {
    println!("akiri_disponeblajn_seriaportojn");
}
