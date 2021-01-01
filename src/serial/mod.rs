use std::fmt::{Display, Formatter};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serialport::{open_with_settings, SerialPort};

use crate::app::{error, CerialState};
use crate::ui::DisplayUpdateEvent;

/// Serial port telemetry
#[derive(Debug, Copy, Clone)]
pub struct SerialTelemetry {
    /// Clear to send
    cts: bool,
    /// Carrier detect
    cd: bool,
    /// Ring indicator
    ri: bool,
    /// Data set ready
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
    /// Read serial telemetry from a serial port
    fn read_serial_telemetry(serial_port: &mut Box<dyn SerialPort>) -> SerialTelemetry {
        Self {
            cts: serial_port.read_clear_to_send().unwrap_or(false),
            cd: serial_port.read_carrier_detect().unwrap_or(false),
            ri: serial_port.read_ring_indicator().unwrap_or(false),
            dsr: serial_port.read_ring_indicator().unwrap_or(false),
        }
    }
}

/// Serial RX Thread
///
/// Handles reading data and telemetry from a serial port
pub fn serial_rx_thread(
    tx: Sender<DisplayUpdateEvent>,
    serial_port: Arc<Mutex<Box<dyn SerialPort>>>,
) {
    loop {
        // Grab lock on serial port
        if let Ok(mut serial_port) = serial_port.lock() {
            // Init buffer
            let mut buffer = vec![0; 128];
            // Block for read (timeout has also been set)
            match serial_port.read(&mut buffer) {
                Ok(count) => {
                    // Try to send all bytes received to the display loop
                    if tx
                        .send(DisplayUpdateEvent::SerialInput(
                            buffer.drain(..count).collect(),
                        ))
                        .is_err()
                    {
                        // Break if the send fails
                        break;
                    }
                }
                Err(err) => match err.kind() {
                    // Ignore timout
                    std::io::ErrorKind::TimedOut => {}
                    // On error, print error and exit
                    err => {
                        println!("Error {:?}", err);
                        break;
                    }
                },
            };

            // Get telemetry data
            if tx
                .send(DisplayUpdateEvent::SerialTelemetry(
                    SerialTelemetry::read_serial_telemetry(&mut serial_port),
                ))
                .is_err()
            {
                // Break if send fails
                break;
            }
        }

        // Sleep to allow other threads to grab the serial port
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// Serial TX thread
///
/// Handles writing data to a serial port
pub fn serial_tx_thread(rx: Receiver<Vec<u8>>, serial_port: Arc<Mutex<Box<dyn SerialPort>>>) {
    // Wait for data to be available
    while let Ok(buffer) = rx.recv() {
        // Lock serial port
        if let Ok(mut serial_port) = serial_port.lock() {
            // Try and write all data, break if there is an error
            if serial_port.write_all(buffer.as_slice()).is_err() {
                break;
            }
        } else {
            // Break if mutex lock fails
            break;
        }
    }
}

/// Open a serial port based on the application state
pub fn open_serial_port(
    cerial_state: &CerialState,
) -> error::Result<Arc<Mutex<Box<dyn SerialPort>>>> {
    let serialport = open_with_settings(&cerial_state.serial_dev, &cerial_state.serial_settings)?;
    Ok(Arc::new(Mutex::new(serialport)))
}
