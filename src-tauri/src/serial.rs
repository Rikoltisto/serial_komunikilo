use serde::Serialize;
use tauri::{State, ipc::Channel};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf},
    sync::Mutex, task::JoinHandle,
};
use tokio_serial::SerialStream;

mod serial_manager;
mod settings;

use serial_manager::SerialManager;
use settings::Settings;

pub struct GlobalSerialManager(
    pub Mutex<Option<ReadHalf<SerialStream>>>,
    pub Mutex<Option<WriteHalf<SerialStream>>>,
);

#[derive(Serialize)]
pub struct ReceivePayload {
    content: Vec<u8>,
}

#[tauri::command]
pub async fn open_serial_port(
    s: Settings,
    global_serial_manager: State<'_, GlobalSerialManager>,
) -> Result<(), ()> {
    let mut manager = SerialManager::new().setup_settings(s).open();

    let (read_half, write_half) = manager.split();

    *global_serial_manager.0.lock().await = Some(read_half);
    *global_serial_manager.1.lock().await = Some(write_half);

    Ok(())
}

#[tauri::command]
pub async fn close_serial_port(app: tauri::AppHandle,) {
    app.restart();
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

#[tauri::command]
pub async fn send_serial_data(
    data: Vec<u8>,
    global_serial_manager: State<'_, GlobalSerialManager>,
) -> Result<(), ()> {
    global_serial_manager
        .1
        .lock()
        .await
        .as_mut()
        .unwrap()
        .write_all(data.as_slice())
        .await
        .unwrap();

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn read_serial_data(
    global_serial_manager: State<'_, GlobalSerialManager>,
    on_receive_payload: Channel<ReceivePayload>,
) -> Result<(), ()> {
    let mut reader = global_serial_manager.0.lock().await.take();

    tokio::spawn(async move {
        let mut buffer = [0u8; 1024];

        loop {
            let bytes_read = reader.as_mut().unwrap().read(&mut buffer).await.unwrap();
            if bytes_read > 0 {
                on_receive_payload.send(ReceivePayload {
                    content: buffer[..bytes_read].to_vec(),
                }).unwrap();
            }
        }
    });

    Ok(())
}
