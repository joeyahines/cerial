use crate::serial::SerialTelemetry;
use crossterm::event;
use crossterm::event::{Event, KeyEvent};
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};

pub enum DisplayUpdateEvent {
    KeyInput(KeyEvent),
    SerialInput(Vec<u8>),
    SerialTelemetry(SerialTelemetry),
    Tick,
}

pub fn terminal_event_thread(tx: Sender<DisplayUpdateEvent>) {
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();
    loop {
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                tx.send(DisplayUpdateEvent::KeyInput(key)).unwrap();
            }
        }
        if last_tick.elapsed() >= tick_rate {
            if tx.send(DisplayUpdateEvent::Tick).is_err() {
                break;
            }
            last_tick = Instant::now();
        }
    }
}
