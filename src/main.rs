use std::error::Error;
use std::io;
use std::io::Write;
use std::sync::mpsc::channel;
use std::thread;

use crossterm::{
    event::KeyCode,
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    cursor::{MoveTo, RestorePosition, SavePosition}
};
use serialport::open_with_settings;
use structopt::StructOpt;

use app::{CerialMode, CerialState};
use args::Cerial;
use serial::serial_rx_thread;
use ui::{DisplayUpdateEvent, terminal_event_thread};

mod ui;
mod app;
mod serial;
mod args;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Cerial = Cerial::from_args();
    let serial_settings = args.clone().into();
    let mut cerial_state = CerialState {
        input: String::new(),
        mode: CerialMode::Menu,
        exit: false
    };

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout,
        EnterAlternateScreen,
        SavePosition,
        MoveTo(0, 0)
    )?;

    let serialport = open_with_settings(&args.serial_port, &serial_settings).unwrap();

    let (term_display_update_tx, display_update_rx) = channel();
    let serial_display_update_tx = term_display_update_tx.clone();

    thread::spawn(move || { terminal_event_thread(term_display_update_tx)});
    thread::spawn(move || {serial_rx_thread(serial_display_update_tx, serialport)});

    loop {
        match display_update_rx.recv()? {
            DisplayUpdateEvent::KeyInput(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(
                        stdout,
                        RestorePosition,
                        LeaveAlternateScreen,
                    )?;
                    break;
                }
                KeyCode::Char(c) => cerial_state.input.push(c),
                _ => {}
            },
            DisplayUpdateEvent::Tick => {

            }
            DisplayUpdateEvent::SerialInput(data) => {
                let string: String = String::from_utf8(data).unwrap();
                execute!(
                    stdout,
                    Print(string)
                ).unwrap();
            }
        }
    }

    Ok(())
}