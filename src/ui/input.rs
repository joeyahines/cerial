use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn key_event_to_vec(key_event: KeyEvent) -> Vec<u8> {
    match key_event.code {
        KeyCode::Char(c) => {
            let c = c as u8;

            if key_event.modifiers == KeyModifiers::CONTROL {
                [c + 0x01 - b'a'].to_vec()
            } else {
                [c as u8].to_vec()
            }
        }
        KeyCode::Backspace => b"\x7f".to_vec(),
        KeyCode::Enter => b"\n".to_vec(),
        KeyCode::Left => match key_event.modifiers {
            KeyModifiers::NONE => b"\x1b[[D".to_vec(),
            KeyModifiers::SHIFT => b"\x1b[[1;2D".to_vec(),
            KeyModifiers::CONTROL => b"\x1b[[1;5D".to_vec(),
            KeyModifiers::ALT => b"\x1b[[1;3D".to_vec(),
            _ => b"".to_vec(),
        },
        KeyCode::Right => match key_event.modifiers {
            KeyModifiers::NONE => b"\x1b[[C".to_vec(),
            KeyModifiers::SHIFT => b"\x1b[[1;2C".to_vec(),
            KeyModifiers::CONTROL => b"\x1b[[1;5C".to_vec(),
            KeyModifiers::ALT => b"\x1b[[1;3C".to_vec(),
            _ => b"".to_vec(),
        },
        KeyCode::Up => match key_event.modifiers {
            KeyModifiers::NONE => b"\x1b[[A".to_vec(),
            KeyModifiers::SHIFT => b"\x1b[[1;2A".to_vec(),
            KeyModifiers::CONTROL => b"\x1b[[1;5A".to_vec(),
            KeyModifiers::ALT => b"\x1b[[1;3A".to_vec(),
            _ => b"".to_vec(),
        },
        KeyCode::Down => match key_event.modifiers {
            KeyModifiers::NONE => b"\x1b[[B".to_vec(),
            KeyModifiers::SHIFT => b"\x1b[[1;2B".to_vec(),
            KeyModifiers::CONTROL => b"\x1b[[1;5B".to_vec(),
            KeyModifiers::ALT => b"\x1b[[1;3B".to_vec(),
            _ => b"".to_vec(),
        },
        KeyCode::Home => match key_event.modifiers {
            KeyModifiers::NONE => b"\x1b[[1~".to_vec(),
            KeyModifiers::SHIFT => b"\x1b[[1;2H".to_vec(),
            KeyModifiers::CONTROL => b"\x1b[[1;5H".to_vec(),
            KeyModifiers::ALT => b"\x1b[[1;3H".to_vec(),
            _ => b"".to_vec(),
        },
        KeyCode::End => match key_event.modifiers {
            KeyModifiers::NONE => b"\x1b[[4~".to_vec(),
            KeyModifiers::SHIFT => b"\x1b[[1;2f".to_vec(),
            KeyModifiers::CONTROL => b"\x1b[[1;5f".to_vec(),
            KeyModifiers::ALT => b"\x1b[[1;3f".to_vec(),
            _ => b"".to_vec(),
        },
        KeyCode::PageUp => match key_event.modifiers {
            KeyModifiers::NONE => b"\x1b[[5~".to_vec(),
            KeyModifiers::CONTROL => b"\x1b[[5;5~".to_vec(),
            KeyModifiers::ALT => b"\x1b[[5;3~".to_vec(),
            _ => b"".to_vec(),
        },
        KeyCode::PageDown => match key_event.modifiers {
            KeyModifiers::NONE => b"\x1b[[6~".to_vec(),
            KeyModifiers::CONTROL => b"\x1b[[6;5~".to_vec(),
            KeyModifiers::ALT => b"\x1b[[6;3~".to_vec(),
            _ => b"".to_vec(),
        },
        KeyCode::Tab => match key_event.modifiers {
            KeyModifiers::NONE => b"\t".to_vec(),
            KeyModifiers::SHIFT => b"\x1b[[Z".to_vec(),
            _ => b"".to_vec(),
        },
        KeyCode::BackTab => b"\x1b[[Z".to_vec(),
        KeyCode::Delete => match key_event.modifiers {
            KeyModifiers::NONE => b"\x1b[[3~".to_vec(),
            KeyModifiers::CONTROL => b"\x1b[[3;5~".to_vec(),
            KeyModifiers::ALT => b"\x1b[[3;3~".to_vec(),
            _ => b"".to_vec(),
        },
        KeyCode::Insert => match key_event.modifiers {
            KeyModifiers::NONE => b"\x1b[[2~".to_vec(),
            KeyModifiers::CONTROL => b"\x1b[[2;5~".to_vec(),
            KeyModifiers::ALT => b"\x1b[[2;3~".to_vec(),
            _ => b"".to_vec(),
        },
        KeyCode::F(n) => match n {
            1 => b"\x1b[[OP".to_vec(),
            2 => b"\x1b[[OQ".to_vec(),
            3 => b"\x1b[[OR".to_vec(),
            4 => b"\x1b[[OS".to_vec(),
            5 => b"\x1b[[15~".to_vec(),
            6 => b"\x1b[[17~".to_vec(),
            7 => b"\x1b[[18~".to_vec(),
            8 => b"\x1b[[19~".to_vec(),
            9 => b"\x1b[[20~".to_vec(),
            10 => b"\x1b[[21~".to_vec(),
            11 => b"\x1b[[22~".to_vec(),
            12 => b"\x1b[[24~".to_vec(),
            _ => b"".to_vec(),
        },
        KeyCode::Null => b"\0".to_vec(),
        KeyCode::Esc => b"\x1b[[".to_vec(),
    }
}
