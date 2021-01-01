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

/// Terminal Event Thread
///
/// Sends keyboard and terminal events to the display loop
pub fn terminal_event_thread(tx: Sender<DisplayUpdateEvent>) {
    let tick_rate = Duration::from_millis(100);
    loop {
        // Poll for event with a timeout of tick rate
        if event::poll(tick_rate).unwrap() {
            // Read event, should not block
            if let Event::Key(key) = event::read().unwrap() {
                // Send event to display thread
                tx.send(DisplayUpdateEvent::KeyInput(key)).unwrap();
            }
        // If no event is ready, send a ping event to the display loop
        } else if tx.send(DisplayUpdateEvent::Ping).is_err() {
            // If there was an error, break
            break;
        }
    }
}
