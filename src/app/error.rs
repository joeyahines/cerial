use std::fmt::{Debug, Display, Formatter};
use std::sync::mpsc;

pub type Result<T> = std::result::Result<T, CerialError>;

#[derive(Debug)]
pub enum CerialError {
    SerialError(serialport::Error),
    CrosstermError(crossterm::ErrorKind),
    ThreadMessageError(mpsc::RecvError),
    IOError(std::io::Error),
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
