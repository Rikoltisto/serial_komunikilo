use serial_manager::SerialManager;
use tauri::State;

mod serial_manager;
mod settings;

use settings::Settings;
use std::sync::Mutex;

pub struct GlobalSerialManager(pub Mutex<Option<SerialManager>>);

#[tauri::command]
pub async fn open_serial_port(
    s: Settings,
    global_serial_manager: State<'_, GlobalSerialManager>,
) -> Result<(), ()> {
    let mut manager = SerialManager::new().setup_settings(s);
    manager.open();

    *global_serial_manager.0.lock().unwrap() = Some(manager);

    Ok(())
}

#[tauri::command]
pub async fn close_serial_port(global_serial_manager: State<'_, GlobalSerialManager>,) -> Result<(), ()> {
    if let Some(mut manager) = global_serial_manager.0.lock().unwrap().take() {
        manager.close();
    };

    Ok(())
}

#[tauri::command]
pub async fn get_all_serial_port() -> Option<Vec<String>> {
    let ports = serialport::available_ports().unwrap();

    let mut port_names: Vec<String> = Vec::new();

    for port in ports {
        port_names.push(port.port_name);
    }

    Some(port_names)
}
