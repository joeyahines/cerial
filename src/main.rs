use std::io;
use std::io::Write;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use crossterm::event::KeyEvent;
use crossterm::terminal::{size, ClearType};
use crossterm::tty::IsTty;
use crossterm::{
    cursor::{MoveTo, RestorePosition, SavePosition},
    event::{KeyCode, KeyModifiers},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, EnableLineWrap, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use serialport::{open_with_settings, SerialPort};
use structopt::StructOpt;

use app::error::Result;
use app::{CerialMode, CerialState};
use args::Cerial;
use serial::{serial_rx_thread, serial_tx_thread};
use ui::{terminal_event_thread, DisplayUpdateEvent};

use crate::app::error::CerialError;
use crate::app::MenuState;

mod app;
mod args;
mod serial;
mod ui;

fn menu_mode<T: Write>(
    app_state: &mut CerialState,
    _stream: &mut T,
    key_event: KeyEvent,
) -> Result<()> {
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
        KeyEvent {
            code: KeyCode::Char('m'),
            modifiers: KeyModifiers::NONE,
        } => {
            app_state.menu_state = match app_state.menu_state {
                MenuState::Hidden => MenuState::SerialSettings,
                MenuState::SerialSettings => MenuState::SerialTelemetry,
                MenuState::SerialTelemetry => MenuState::Hidden,
            };
        }
        KeyEvent { .. } => {}
    };

    Ok(())
}

fn insert_mode<T: Write>(
    app_state: &mut CerialState,
    _stream: &mut T,
    key_event: KeyEvent,
    serial_send_tx: &Sender<Vec<u8>>,
) -> Result<()> {
    match key_event {
        KeyEvent {
            code: KeyCode::Char('5'),
            modifiers: KeyModifiers::CONTROL,
        } => {
            app_state.mode = CerialMode::Menu;
        }
        KeyEvent { .. } => {
            if let KeyCode::Char(c) = key_event.code {
                serial_send_tx.send(vec![c as u8]).unwrap();
            }
        }
    };

    Ok(())
}

fn clear_menu_bar<T: Write>(stream: &mut T) -> Result<()> {
    let (_, rows) = size()?;
    execute!(
        stream,
        SavePosition,
        MoveTo(0, rows),
        Clear(ClearType::CurrentLine),
        RestorePosition
    )
    .map_err(|e| e.into())
}

fn print_menu_bar<T: Write>(app_state: &CerialState, stream: &mut T) -> Result<()> {
    let (_, rows) = size()?;
    execute!(
        stream,
        SavePosition,
        MoveTo(0, rows),
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::White),
        Print(app_state.menu_string()),
        RestorePosition
    )
    .map_err(|e| e.into())
}

fn open_serial_port(cerial_state: &CerialState) -> Result<Arc<Mutex<Box<dyn SerialPort>>>> {
    let serialport = open_with_settings(&cerial_state.serial_dev, &cerial_state.serial_settings)?;
    Ok(Arc::new(Mutex::new(serialport)))
}

fn main() -> Result<()> {
    let args: Cerial = Cerial::from_args();
    let serial_settings = args.clone().into();
    let mut cerial_state = CerialState::default()
        .update_serial_settings(serial_settings)
        .update_serial_dev(args.serial_port.to_str().unwrap());

    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    if !stdout.is_tty() {
        return Err(CerialError::NotTTY);
    }

    enable_raw_mode()?;

    let serialport_read = open_serial_port(&cerial_state)?;
    let serialport_write = serialport_read.clone();

    execute!(
        stdout,
        EnterAlternateScreen,
        Clear(ClearType::All),
        MoveTo(0, 0),
        EnableLineWrap,
    )?;

    let (term_display_update_tx, display_update_rx) = channel();
    let serial_display_update_tx = term_display_update_tx.clone();

    let (serial_send_tx, serial_send_rx) = channel();

    let term_event_thread = thread::spawn(move || terminal_event_thread(term_display_update_tx));
    let serial_rx_thread =
        thread::spawn(move || serial_rx_thread(serial_display_update_tx, serialport_read));
    let serial_tx_thread =
        thread::spawn(move || serial_tx_thread(serial_send_rx, serialport_write));

    print_menu_bar(&cerial_state, &mut stderr)?;
    while !cerial_state.exit {
        match display_update_rx.recv()? {
            DisplayUpdateEvent::KeyInput(event) => {
                clear_menu_bar(&mut stderr)?;
                match cerial_state.mode {
                    CerialMode::Menu => menu_mode(&mut cerial_state, &mut stdout, event)?,
                    CerialMode::Input => {
                        insert_mode(&mut cerial_state, &mut stdout, event, &serial_send_tx)?
                    }
                    _ => {}
                };
            }
            DisplayUpdateEvent::SerialInput(data) => {
                clear_menu_bar(&mut stderr)?;
                stdout.write_all(data.as_slice())?;
                stdout.flush()?;
            }
            DisplayUpdateEvent::SerialTelemetry(tel) => {
                cerial_state.serial_telemetry = tel;
            }
            _ => {}
        }
        print_menu_bar(&cerial_state, &mut stderr)?;
    }
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen,)?;

    // Cleanup
    std::mem::drop(display_update_rx);
    std::mem::drop(serial_send_tx);
    term_event_thread.join().unwrap();
    serial_rx_thread.join().unwrap();
    serial_tx_thread.join().unwrap();

    Ok(())
}
