use serialport::SerialPort;
use std::time::Duration;

use crate::serial::settings::Settings;

pub struct SerialManager {
    pub port: Option<Box<dyn SerialPort>>,
    settings: Settings,
}

impl SerialManager {
    pub fn new() -> Self {
        Self {
            port: None,
            settings: Settings::new(),
        }
    }

    pub fn open(&mut self) {
        let (data_bits, stop_bits, parity) = self.settings.to_serialport_settings();

        let port = serialport::new(&self.settings.selected_serial_port, self.settings.baud_rate)
            .data_bits(data_bits)
            .stop_bits(stop_bits)
            .parity(parity)
            .timeout(Duration::from_millis(1000))
            .open()
            .unwrap();

        self.port = Some(port);
    }

    pub fn setup_settings(mut self, s: Settings) -> Self {
        self.settings = s;
        self
    }

    pub fn close(&mut self) {
        self.port.take();
    }
}
