pub mod input;

use crate::serial::SerialTelemetry;
use crossterm::event;
use crossterm::event::{Event, KeyEvent};
use std::sync::mpsc::Sender;
use std::time::Duration;

/// Display update event
pub enum DisplayUpdateEvent {
    /// Ping message to check if the channel is open
    Ping,
    /// Key input event
    KeyInput(KeyEvent),
    /// Serial data input
    SerialInput(Vec<u8>),
    /// Serial telemetry update event
    SerialTelemetry(SerialTelemetry),
    /// Terminal update event (cols, rows)
    TerminalResize(u16, u16),
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
            let event = match event::read().unwrap() {
                Event::Key(key) => DisplayUpdateEvent::KeyInput(key),
                Event::Mouse(_) => DisplayUpdateEvent::Ping,
                Event::Resize(cols, rows) => DisplayUpdateEvent::TerminalResize(cols, rows),
            };
            tx.send(event).unwrap();
        // If no event is ready, send a ping event to the display loop
        } else if tx.send(DisplayUpdateEvent::Ping).is_err() {
            // If there was an error, break
            break;
        }
    }
}
