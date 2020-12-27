use crate::ui::DisplayUpdateEvent;
use serialport::SerialPort;
use std::fmt::{Display, Formatter};
use std::sync::mpsc::Sender;

#[derive(Debug, Copy, Clone)]
pub struct SerialTelemetry {
    /// clear to send
    cts: bool,
    /// carrier detect
    cd: bool,
    /// ring indicator
    ri: bool,
    /// data set ready
    dsr: bool,
}

impl Display for SerialTelemetry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CTS: {} CD: {} RI: {} DSR: {}",
            self.cts, self.cd, self.ri, self.dsr
        )
    }
}

impl Default for SerialTelemetry {
    fn default() -> Self {
        Self {
            cts: false,
            cd: false,
            ri: false,
            dsr: false,
        }
    }
}

impl SerialTelemetry {
    fn read_serial_telemetry(serial_port: &mut Box<dyn SerialPort>) -> SerialTelemetry {
        Self {
            cts: serial_port.read_clear_to_send().unwrap_or(false),
            cd: serial_port.read_carrier_detect().unwrap_or(false),
            ri: serial_port.read_ring_indicator().unwrap_or(false),
            dsr: serial_port.read_ring_indicator().unwrap_or(false),
        }
    }
}

pub fn serial_rx_thread(tx: Sender<DisplayUpdateEvent>, mut serial_port: Box<dyn SerialPort>) {
    loop {
        let mut buffer = vec![0; 128];
        match serial_port.read(&mut buffer) {
            Ok(_) => {
                if tx
                    .send(DisplayUpdateEvent::SerialInput(buffer.drain(..).collect()))
                    .is_err()
                {
                    break;
                }
            }
            Err(err) => match err.kind() {
                std::io::ErrorKind::TimedOut => {}
                err => {
                    println!("Error {:?}", err);
                    break;
                }
            },
        };

        if tx
            .send(DisplayUpdateEvent::SerialTelemetry(
                SerialTelemetry::read_serial_telemetry(&mut serial_port),
            ))
            .is_err()
        {
            break;
        }
    }
}
