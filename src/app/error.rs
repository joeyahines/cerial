use std::fmt::{Debug, Display, Formatter};
use std::sync::mpsc;

/// Cerial result type
pub type Result<T> = std::result::Result<T, CerialError>;

/// Cerial Error
#[derive(Debug)]
pub enum CerialError {
    /// Serial port error
    SerialError(serialport::Error),
    /// Crossterm Error
    CrosstermError(crossterm::ErrorKind),
    /// MPSC messaging error
    ThreadMessageError(mpsc::RecvError),
    /// IO Error
    IOError(std::io::Error),
    /// Terminal not a TTY error
    NotTTY,
}

impl From<crossterm::ErrorKind> for CerialError {
    fn from(e: crossterm::ErrorKind) -> Self {
        Self::CrosstermError(e)
    }
}

impl From<serialport::Error> for CerialError {
    fn from(e: serialport::Error) -> Self {
        Self::SerialError(e)
    }
}

impl From<mpsc::RecvError> for CerialError {
    fn from(e: mpsc::RecvError) -> Self {
        Self::ThreadMessageError(e)
    }
}

impl From<std::io::Error> for CerialError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}

impl Display for CerialError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            CerialError::SerialError(e) => e.to_string(),
            CerialError::CrosstermError(e) => e.to_string(),
            CerialError::ThreadMessageError(e) => e.to_string(),
            CerialError::IOError(e) => e.to_string(),
            CerialError::NotTTY => "Terminal is not TTY compatible".to_string(),
        };

        write!(f, "Cerial Error: {}", msg)
    }
}
