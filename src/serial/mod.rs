use crate::ui::DisplayUpdateEvent;
use std::sync::mpsc::Sender;
use serialport::SerialPort;

pub fn serial_rx_thread(tx: Sender<DisplayUpdateEvent>, mut serial_port: Box<dyn SerialPort>) {
    let mut in_char = vec![0; 256];
    loop {
        match serial_port.read(&mut in_char) {
            Ok(_) => {
                if tx.send(DisplayUpdateEvent::SerialInput(in_char.clone())).is_err() {
                    break;
                }
            },
            Err(err) => {
                match err.kind() {
                    std::io::ErrorKind::TimedOut => {},
                    err => {
                        println!("Error {:?}", err);
                        break;
                    }
                }
            }
        };
    }
}
