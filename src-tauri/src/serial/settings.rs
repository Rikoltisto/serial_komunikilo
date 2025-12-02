use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub selected_serial_port: String,
    pub baud_rate: u32,
    data_bits: u8,
    stop_bits: u8,
    parity: u8,
}

impl Settings {
    pub fn to_serialport_settings(
        &self,
    ) -> (
        serialport::DataBits,
        serialport::StopBits,
        serialport::Parity,
    ) {
        let data_bits = match self.data_bits {
            8 => serialport::DataBits::Eight,
            7 => serialport::DataBits::Seven,
            6 => serialport::DataBits::Six,
            5 => serialport::DataBits::Five,
            _ => serialport::DataBits::Eight,
        };

        let stop_bits = match self.stop_bits {
            1 => serialport::StopBits::One,
            2 => serialport::StopBits::Two,
            _ => serialport::StopBits::One,
        };

        let parity = match self.parity {
            0 => serialport::Parity::None,
            1 => serialport::Parity::Even,
            2 => serialport::Parity::Odd,
            _ => serialport::Parity::None,
        };

        (data_bits, stop_bits, parity)
    }

    pub fn new() -> Self {
        Self {
            selected_serial_port: String::new(),
            baud_rate: 115200,
            data_bits: 8,
            stop_bits: 1,
            parity: 0,
        }
    }
}
