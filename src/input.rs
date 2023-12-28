use crate::app::{App, Tabs};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::io;

use crate::app::AppAction;
use std::collections::HashMap;

// TODO invalid input feedback
pub fn handle_events(
    app: &mut App,
    timer_setting_tab_keymap: &HashMap<KeyEvent, AppAction>,
    task_manager_keymap: &HashMap<KeyEvent, AppAction>,
) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(50))? {
        match event::read()? {
            Event::Key(key) => {
                return handle_key(key, app, timer_setting_tab_keymap, task_manager_keymap);
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
    Ok(())
}

fn handle_key(
    key: KeyEvent,
    app: &mut App,
    tiemr_setting_keymap: &HashMap<KeyEvent, AppAction>,
    task_manager_keymap: &HashMap<KeyEvent, AppAction>,
) -> io::Result<()> {
    match app.tab_selected {
        Tabs::TaskManager => {
            if let Some(action) = task_manager_keymap.get(&key) {
                action(app);
            } else {
                // NOTE these keys has no special mapping
                // if they are valid setting input, we push them to screen
                if let KeyCode::Char(code) = key.code {
                    if code.is_alphanumeric() {
                        app.push_user_input_field(code);
                    }
                }
            }
        }
        Tabs::PomodoroSetting => {
            if let Some(action) = tiemr_setting_keymap.get(&key) {
                action(app);
            } else {
                // NOTE these keys has no special mapping
                // if they are valid setting input, we push them to screen
                if let KeyCode::Char(code) = key.code {
                    if code.is_ascii_digit() {
                        app.push_user_input_field(code);
                    }
                }
            }
        }
    }
    Ok(())
}
