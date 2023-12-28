use crate::app::{App, Tabs};
use crossterm::event::{self, Event, KeyEvent};
use std::io;

use std::collections::HashMap;
use crate::app::AppAction;

// TODO invalid input feedback
pub fn handle_events(app: &mut App, keymap: &HashMap<KeyEvent, AppAction>) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(50))? {
        match event::read()? {
            Event::Key(key) => {
                return handle_key(key, app, keymap);
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
    keymap: &HashMap<KeyEvent, AppAction>,
) -> io::Result<()> {
    match app.tab_selected {
        Tabs::TaskManager => Ok(()),
        Tabs::PomodoroSetting => {
            if let Some(action) = keymap.get(&key) {
                action(app);
            }
            Ok(())
        }
    }
}

