use std::error::Error;
use std::io;
use std::io::{Stdout, Write};
use std::sync::mpsc::channel;
use std::thread;

use crossterm::event::KeyEvent;
use crossterm::terminal::{size, ClearType};
use crossterm::{
    cursor::{MoveTo, RestorePosition, SavePosition},
    event::{KeyCode, KeyModifiers},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use serialport::open_with_settings;
use structopt::StructOpt;

use app::{CerialMode, CerialState};
use args::Cerial;
use serial::serial_rx_thread;
use ui::{terminal_event_thread, DisplayUpdateEvent};

mod app;
mod args;
mod serial;
mod ui;

fn menu_mode(
    app_state: &mut CerialState,
    _stdout: &mut Stdout,
    key_event: KeyEvent,
) -> crossterm::Result<()> {
    match key_event {
        KeyEvent {
            code: KeyCode::Char('i'),
            modifiers: KeyModifiers::NONE,
        } => {
            app_state.mode = CerialMode::Input;
        }
        KeyEvent {
            code: KeyCode::Char('h'),
            modifiers: KeyModifiers::NONE,
        } => {
            app_state.mode = CerialMode::HexInput;
        }
        KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
        } => {
            app_state.exit = true;
        }
        KeyEvent { .. } => {}
    };

    Ok(())
}

fn insert_mode(
    app_state: &mut CerialState,
    _stdout: &mut Stdout,
    key_event: KeyEvent,
) -> crossterm::Result<()> {
    match key_event {
        KeyEvent {
            code: KeyCode::Char('5'),
            modifiers: KeyModifiers::CONTROL,
        } => {
            app_state.mode = CerialMode::Menu;
            Ok(())
        }
        KeyEvent { .. } => Ok(()),
    }
}

fn clear_menu_bar(stdout: &mut Stdout) -> crossterm::Result<()> {
    let (_, rows) = size()?;
    execute!(
        stdout,
        SavePosition,
        MoveTo(0, rows),
        Clear(ClearType::CurrentLine),
        RestorePosition
    )
}

fn print_menu_bar(app_state: &CerialState, stdout: &mut Stdout) -> crossterm::Result<()> {
    let (_, rows) = size()?;
    execute!(
        stdout,
        SavePosition,
        MoveTo(0, rows),
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::White),
        Print(format!("{} {}", app_state.mode, app_state.serial_telemetry)),
        RestorePosition
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Cerial = Cerial::from_args();
    let serial_settings = args.clone().into();
    let mut cerial_state = CerialState::default();

    let serialport = open_with_settings(&args.serial_port, &serial_settings).unwrap();

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        Clear(ClearType::All),
        MoveTo(0, 0)
    )?;

    let (term_display_update_tx, display_update_rx) = channel();
    let serial_display_update_tx = term_display_update_tx.clone();

    thread::spawn(move || terminal_event_thread(term_display_update_tx));
    thread::spawn(move || serial_rx_thread(serial_display_update_tx, serialport));

    while !cerial_state.exit {
        match display_update_rx.recv()? {
            DisplayUpdateEvent::KeyInput(event) => {
                match cerial_state.mode {
                    CerialMode::Menu => menu_mode(&mut cerial_state, &mut stdout, event)?,
                    CerialMode::Input => insert_mode(&mut cerial_state, &mut stdout, event)?,
                    _ => {}
                };
            }
            DisplayUpdateEvent::Tick => {
                clear_menu_bar(&mut stdout)?;
                print_menu_bar(&cerial_state, &mut stdout)?;
            }
            DisplayUpdateEvent::SerialInput(data) => {
                stdout.write_all(data.as_slice())?;
            }
            DisplayUpdateEvent::SerialTelemetry(tel) => {
                cerial_state.serial_telemetry = tel;
            }
        }
    }
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen,)?;

    Ok(())
}
