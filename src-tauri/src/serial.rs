fn open_serial_port() {}

fn close_serial_port() {}

#[tauri::command]
pub async fn get_all_serial_port() -> Option<Vec<String>> {
    let ports = serialport::available_ports().unwrap();

    let mut port_names: Vec<String> = Vec::new();

    for port in ports {
        port_names.push(port.port_name);
    }

    Some(port_names)
}
