use crate::serial::SerialTelemetry;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub enum CerialMode {
    Menu,
    Input,
    HexInput,
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
    pub input: String,
    pub mode: CerialMode,
    pub exit: bool,
    pub serial_telemetry: SerialTelemetry,
}
