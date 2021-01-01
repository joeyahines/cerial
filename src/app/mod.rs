pub mod error;

use crate::serial::SerialTelemetry;
use serialport::SerialPortSettings;
use std::fmt::{Display, Formatter};

/// Application state
#[derive(Debug, Copy, Clone)]
pub enum CerialMode {
    /// Menu mode
    Menu,
    /// Normal input mode
    Input,
    /// Hex input mode
    HexInput,
}

/// Menu state
#[derive(Debug, Copy, Clone)]
pub enum MenuState {
    /// Menu line is hidden
    Hidden,
    /// Displays current serial settings
    SerialSettings,
    /// Display serial port telemetry
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

/// Struct respresing the application state
#[derive(Debug, Clone, Default)]
pub struct CerialState {
    /// Application mode
    pub mode: CerialMode,
    /// Exit flag
    pub exit: bool,
    /// Serial telemetry
    pub serial_telemetry: SerialTelemetry,
    /// Path to serial device
    pub serial_dev: String,
    /// Current serial settings
    pub serial_settings: SerialPortSettings,
    /// Current menu state
    pub menu_state: MenuState,
}

impl CerialState {
    /// Update serial settings
    /// **Note** Does not update the settings of physical serial port
    pub fn update_serial_settings(mut self, settings: SerialPortSettings) -> Self {
        self.serial_settings = settings;
        self
    }

    /// Update serial device
    /// **Note** Does not change the target serial port
    pub fn update_serial_dev(mut self, dev: &str) -> Self {
        self.serial_dev = dev.to_string();
        self
    }

    /// Get the serial settings display string
    fn display_serial_settings(&self) -> String {
        format!("{} {}", self.serial_dev, self.serial_settings.baud_rate)
    }

    /// Get the current menu line
    pub fn menu_string(&self) -> String {
        match self.menu_state {
            MenuState::Hidden => String::new(),
            MenuState::SerialSettings => {
                format!("{}: {}", self.mode, self.display_serial_settings())
            }
            MenuState::SerialTelemetry => format!("{}: {}", self.mode, self.serial_telemetry),
        }
    }
}
