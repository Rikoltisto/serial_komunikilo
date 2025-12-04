use std::time::Duration;
use tokio::io::{split, ReadHalf, WriteHalf};
use tokio_serial::{SerialPortBuilderExt, SerialStream};

use crate::serial::settings::Settings;

pub struct SerialManager {
    port: Option<SerialStream>,
    settings: Settings,
}

impl SerialManager {
    pub fn new() -> Self {
        Self {
            port: None,
            settings: Settings::new(),
        }
    }

    pub fn open(mut self) -> Self {
        let (data_bits, stop_bits, parity) = self.settings.to_serialport_settings();

        let port = tokio_serial::new(&self.settings.selected_serial_port, self.settings.baud_rate)
            .data_bits(data_bits)
            .stop_bits(stop_bits)
            .parity(parity)
            .timeout(Duration::from_millis(1000))
            .open_native_async()
            .unwrap();

        self.port = Some(port);

        self
    }

    pub fn setup_settings(mut self, s: Settings) -> Self {
        self.settings = s;
        self
    }

    pub fn split(&mut self) -> (ReadHalf<SerialStream>, WriteHalf<SerialStream>) {
        let (read_half, write_half) = split(self.port.take().unwrap());

        (read_half, write_half)
    }
}
