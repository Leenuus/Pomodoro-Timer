use crate::app::App;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;

pub fn handle_events(app: &mut App) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        match event::read()? {
            Event::Key(key) => {
                return handle_key(key, app);
            }
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Mouse(_event) => {}
            Event::Paste(_data) => {
                // println!("{:?}", data);
            }
            Event::Resize(_width, _height) => {
                // println!("New size {}x{}", width, height);
            }
        }
    }
    Ok(false)
}

// TODO dynamic keybindings
fn handle_key(key: KeyEvent, app: &mut App) -> io::Result<bool> {
    match (key.kind, key.code, key.modifiers) {
        // we filter user illegal input here
        (KeyEventKind::Press, KeyCode::Char(code), _) if code as u8 >= 48 && code as u8 <= 57 => {
            app.push_user_input_field(code);
            Ok(false)
        }
        (KeyEventKind::Press, KeyCode::Esc, _) => {
            app.clear_input_field();
            Ok(false)
        }
        (KeyEventKind::Press, KeyCode::Backspace, _) => {
            app.pop_user_input_field();
            Ok(false)
        }
        (KeyEventKind::Press, KeyCode::Enter, _) => {
            app.set_timer();
            Ok(false)
        }
        (KeyEventKind::Press, KeyCode::Tab, _) | (KeyEventKind::Press, KeyCode::Down, _) => {
            app.select_next_field();
            Ok(false)
        }
        (KeyEventKind::Press, KeyCode::BackTab, _) | (KeyEventKind::Press, KeyCode::Up, _) => {
            app.select_prev_field();
            Ok(false)
        }
        (KeyEventKind::Press, KeyCode::Char(code), _) => {
            if code == 'q' {
                Ok(true)
            } else if code == 'l' || code == 'h' {
                app.tab_toggle();
                Ok(false)
            } else if code == 'j' {
                app.select_next_field();
                Ok(false)
            } else if code == 'k' {
                app.select_prev_field();
                Ok(false)
            } else if code == 'm' {
                app.abort_timer();
                Ok(false)
            } else if code == 'p' {
                app.launch_timer();
                Ok(false)
            } else if code == 'c' {
                app.pause_timer();
                Ok(false)
            } else if code == 'w' {
                app.task_list.previous_entry();
                Ok(false)
            } else if code == 's' {
                app.task_list.next_entry();
                Ok(false)
            } else if code == ' ' {
                app.toggle_timer();
                Ok(false)
            } else {
                Ok(false)
            }
        }
        _ => Ok(false),
    }
}
