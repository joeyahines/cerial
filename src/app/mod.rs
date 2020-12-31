pub mod error;

use crate::serial::SerialTelemetry;
use serialport::SerialPortSettings;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub enum CerialMode {
    Menu,
    Input,
    HexInput,
}

#[derive(Debug, Copy, Clone)]
pub enum MenuState {
    Hidden,
    SerialSettings,
    SerialTelemetry,
}

impl Default for MenuState {
    fn default() -> Self {
        Self::SerialTelemetry
    }
}

impl Default for CerialMode {
    fn default() -> Self {
        Self::Menu
    }
}

impl Display for CerialMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CerialMode::Menu => "Menu",
            CerialMode::Input => "Input",
            CerialMode::HexInput => "Hex",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CerialState {
    pub mode: CerialMode,
    pub exit: bool,
    pub serial_telemetry: SerialTelemetry,
    pub serial_dev: String,
    pub serial_settings: SerialPortSettings,
    pub menu_state: MenuState,
}

impl CerialState {
    pub fn update_serial_settings(mut self, settings: SerialPortSettings) -> Self {
        self.serial_settings = settings;
        self
    }

    pub fn update_serial_dev(mut self, dev: &str) -> Self {
        self.serial_dev = dev.to_string();
        self
    }

    fn display_serial_settings(&self) -> String {
        format!("{} {}", self.serial_dev, self.serial_settings.baud_rate)
    }

    pub fn menu_string(&self) -> String {
        match self.menu_state {
            MenuState::Hidden => String::new(),
            MenuState::SerialSettings => format!("{}: {}", self.mode, self.display_serial_settings()),
            MenuState::SerialTelemetry => format!("{}: {}", self.mode, self.serial_telemetry),
        }
    }
}
