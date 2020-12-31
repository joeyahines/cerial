use crate::ui::DisplayUpdateEvent;
use serialport::SerialPort;
use std::fmt::{Display, Formatter};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

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

pub fn serial_rx_thread(
    tx: Sender<DisplayUpdateEvent>,
    serial_port: Arc<Mutex<Box<dyn SerialPort>>>,
) {
    loop {
        if let Ok(mut serial_port) = serial_port.lock() {
            let mut buffer = vec![0; 128];
            match serial_port.read(&mut buffer) {
                Ok(count) => {
                    if tx
                        .send(DisplayUpdateEvent::SerialInput(
                            buffer.drain(..count).collect(),
                        ))
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

        std::thread::sleep(Duration::from_millis(10));
    }
}

pub fn serial_tx_thread(rx: Receiver<Vec<u8>>, serial_port: Arc<Mutex<Box<dyn SerialPort>>>) {
    while let Ok(buffer) = rx.recv() {
        if let Ok(mut serial_port) = serial_port.lock() {
            if serial_port.write_all(buffer.as_slice()).is_err() {
                break;
            }
        } else {
            break;
        }
    }
}
