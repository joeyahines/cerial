use crate::serial::SerialTelemetry;
use crossterm::event;
use crossterm::event::{Event, KeyEvent};
use std::sync::mpsc::Sender;
use std::time::Duration;

pub enum DisplayUpdateEvent {
    Ping,
    KeyInput(KeyEvent),
    SerialInput(Vec<u8>),
    SerialTelemetry(SerialTelemetry),
}

pub fn terminal_event_thread(tx: Sender<DisplayUpdateEvent>) {
    let tick_rate = Duration::from_millis(100);
    loop {
        if event::poll(tick_rate).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                tx.send(DisplayUpdateEvent::KeyInput(key)).unwrap();
            }
        }
        else if tx.send(DisplayUpdateEvent::Ping).is_err() {
            break;
        }
    }
}
