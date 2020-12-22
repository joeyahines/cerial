use structopt::StructOpt;
use serialport::{DataBits, FlowControl, Parity, StopBits, Error, ErrorKind, SerialPortSettings};
use std::time::Duration;
use std::str::FromStr;
use std::ffi::OsString;


fn parse_data_bits(src: &str) -> Result<DataBits, Error> {
    let bits = u8::from_str(src).map_err(|f| Error::new(ErrorKind::InvalidInput, f.to_string()))?;
    match bits {
        5 => Ok(DataBits::Five),
        6 => Ok(DataBits::Six),
        7 => Ok(DataBits::Seven),
        8 => Ok(DataBits::Eight),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid data-bit size."))
    }
}

fn parse_flow_control(src: &str) -> Result<FlowControl, Error> {
    match src.to_ascii_lowercase().as_str() {
        "n" => Ok(FlowControl::None),
        "none" => Ok(FlowControl::None),
        "software" => Ok(FlowControl::Software),
        "s" => Ok(FlowControl::Software),
        "h" => Ok(FlowControl::Hardware),
        "hardware" => Ok(FlowControl::Hardware),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid flow control."))
    }
}

fn parse_parity(src: &str) -> Result<Parity, Error> {
    match src.to_ascii_lowercase().as_str() {
        "n" => Ok(Parity::None),
        "none" => Ok(Parity::None),
        "o" => Ok(Parity::Odd),
        "odd" => Ok(Parity::Odd),
        "e" => Ok(Parity::Even),
        "even" => Ok(Parity::Even),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid parity."))
    }
}

fn parse_stop_bits(src: &str) -> Result<StopBits, Error> {
    let bits = u8::from_str(src).map_err(|f| Error::new(ErrorKind::InvalidInput, f.to_string()))?;
    match bits {
        1 => Ok(StopBits::One),
        2 => Ok(StopBits::Two),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid stop bits."))
    }
}


#[derive(Debug, StructOpt, Clone)]
#[structopt(about = "Part of a complete serial breakfast!")]
pub struct Cerial {
    /// Serial port
    pub serial_port: OsString,
    /// Baud rate
    pub baud_rate: u32,
    /// Data bits 5, 6, 7, or 8 bits
    #[structopt(short, long, default_value = "8", parse(try_from_str = parse_data_bits))]
    pub data_bits: DataBits,
    /// Flow control Software, Hardware, or None
    #[structopt(short, long, default_value = "none", parse(try_from_str = parse_flow_control))]
    pub flow_control: FlowControl,
    /// Parity Even, Odd, or, None
    #[structopt(short, long, default_value = "None", parse(try_from_str = parse_parity))]
    pub parity: Parity,
    /// Stop bits 1 or 2
    #[structopt(short, long, default_value = "1", parse(try_from_str = parse_stop_bits))]
    pub stop_bits: StopBits,
    /// Timeout in milliseconds
    #[structopt(short, long, default_value = "10")]
    pub timeout: u64
}

impl Into<SerialPortSettings> for Cerial {
    fn into(self) -> SerialPortSettings {
        SerialPortSettings {
            baud_rate: self.baud_rate,
            data_bits: self.data_bits,
            flow_control: self.flow_control,
            parity: self.parity,
            stop_bits: self.stop_bits,
            timeout: Duration::from_millis(self.timeout)
        }
    }
}