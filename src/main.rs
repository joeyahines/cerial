use std::io;
use std::io::Write;
use std::sync::mpsc::{channel, Receiver, Sender};
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
use structopt::StructOpt;

use app::error::Result;
use app::{CerialMode, CerialState};
use args::CerialArgs;
use serial::{serial_rx_thread, serial_tx_thread};
use ui::{terminal_event_thread, DisplayUpdateEvent};

use crate::app::error::CerialError;
use crate::app::MenuState;
use crate::ui::input::{key_event_to_vec};

mod app;
mod args;
mod serial;
mod ui;

/// Handles user inputs in menu mode
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

/// Handles user input in insert mode
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
            let data = key_event_to_vec(key_event);
            serial_send_tx.send(data).unwrap();
        }
    };

    Ok(())
}

/// Clear the menu bar line
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

/// Display the current menu bar
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

/// Main display loop
fn display_loop(
    mut cerial_state: CerialState,
    display_update_rx: Receiver<DisplayUpdateEvent>,
    serial_send_tx: Sender<Vec<u8>>,
) -> Result<()> {
    // Get stdout and stderr file descs
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    // If stdout is not a tty, exit
    if !stdout.is_tty() {
        return Err(CerialError::NotTTY);
    }

    // Enable raw mode
    enable_raw_mode()?;

    // Prepare terminal for application
    execute!(
        stdout,
        EnterAlternateScreen,
        Clear(ClearType::All),
        MoveTo(0, 0),
        EnableLineWrap,
    )?;

    // Print the menu bar
    print_menu_bar(&cerial_state, &mut stderr)?;

    // Until the user exits
    while !cerial_state.exit {
        // Wait for display update event
        match display_update_rx.recv()? {
            // On a key input
            DisplayUpdateEvent::KeyInput(event) => {
                clear_menu_bar(&mut stderr)?;
                // Handle key input based on state
                match cerial_state.mode {
                    CerialMode::Menu => menu_mode(&mut cerial_state, &mut stdout, event)?,
                    CerialMode::Input => {
                        insert_mode(&mut cerial_state, &mut stdout, event, &serial_send_tx)?
                    }
                    _ => {}
                };
            }
            // On serial input
            DisplayUpdateEvent::SerialInput(data) => {
                // Display data to terminal
                clear_menu_bar(&mut stderr)?;
                stdout.write_all(data.as_slice())?;
                stdout.flush()?;
            }
            //On serial telemetry update
            DisplayUpdateEvent::SerialTelemetry(tel) => {
                // Update current telemetry
                cerial_state.serial_telemetry = tel;
            }
            _ => {}
        }
        // Update meny bar
        print_menu_bar(&cerial_state, &mut stderr)?;
    }

    // Restore terminal to intial state
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen,)?;

    Ok(())
}

fn main() -> Result<()> {
    // Parse args
    let args: CerialArgs = CerialArgs::from_args();
    let serial_settings = args.clone().into();

    // Initialize app state
    let cerial_state = CerialState::default()
        .update_serial_settings(serial_settings)
        .update_serial_dev(args.serial_port.to_str().unwrap());

    // Open serial port
    let serialport_read = serial::open_serial_port(&cerial_state)?;
    let serialport_write = serialport_read.clone();

    // Setup Channels
    let (term_display_update_tx, display_update_rx) = channel();
    let serial_display_update_tx = term_display_update_tx.clone();
    let (serial_send_tx, serial_send_rx) = channel();

    // Start threads
    let term_event_thread = thread::spawn(move || terminal_event_thread(term_display_update_tx));
    let serial_rx_thread =
        thread::spawn(move || serial_rx_thread(serial_display_update_tx, serialport_read));
    let serial_tx_thread =
        thread::spawn(move || serial_tx_thread(serial_send_rx, serialport_write));

    // Begin diisplay
    display_loop(cerial_state, display_update_rx, serial_send_tx)?;

    // Cleanup
    term_event_thread.join().unwrap();
    serial_rx_thread.join().unwrap();
    serial_tx_thread.join().unwrap();

    Ok(())
}
