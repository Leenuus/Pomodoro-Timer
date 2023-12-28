use crate::app::{App, Tabs};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::io;

use std::collections::HashMap;
use crate::app::AppAction;

pub fn handle_events(app: &mut App, keymap: &HashMap<KeyEvent, AppAction>) -> io::Result<bool> {
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
    Ok(false)
}

fn handle_key(
    key: KeyEvent,
    app: &mut App,
    keymap: &HashMap<KeyEvent, AppAction>,
) -> io::Result<bool> {
    match app.tab_selected {
        Tabs::TaskManager => Ok(false),
        Tabs::PomodoroSetting => {
            if let Some(action) = keymap.get(&key) {
                action(app);
            }
            Ok(false)
        }
    }
}

// FIXME a more reasonable keybindings
// HACK keybindings or button to + or - number input
// TODO dynamic keybindings
// fn handle_key(key: KeyEvent, app: &mut App) -> io::Result<bool> {
//     match (key.kind, key.code, key.modifiers) {
//         // we filter user illegal input here
//         (KeyEventKind::Press, KeyCode::Char(code), _) if code as u8 >= 48 && code as u8 <= 57 => {
//             match app.tab_selected {
//                 Tabs::PomodoroSetting => app.push_user_input_field(code),
//                 Tabs::TaskManager => {}
//             };

//             Ok(false)
//         }
//         (KeyEventKind::Press, KeyCode::Esc, _) => {
//             match app.tab_selected {
//                 Tabs::PomodoroSetting => app.clear_input_field(),
//                 Tabs::TaskManager => {}
//             };
//             Ok(false)
//         }
//         (KeyEventKind::Press, KeyCode::Backspace, _) => {
//             match app.tab_selected {
//                 Tabs::PomodoroSetting => app.pop_user_input_field(),
//                 Tabs::TaskManager => {}
//             };
//             Ok(false)
//         }
//         (KeyEventKind::Press, KeyCode::Enter, _) => {
//             match app.tab_selected {
//                 Tabs::PomodoroSetting => app.set_timer(),
//                 // TODO add task
//                 Tabs::TaskManager => {
//                     app.add_task();
//                 }
//             };
//             Ok(false)
//         }
//         (KeyEventKind::Press, KeyCode::Tab, _) | (KeyEventKind::Press, KeyCode::Down, _) => {
//             app.select_next_field();
//             Ok(false)
//         }
//         (KeyEventKind::Press, KeyCode::BackTab, _) | (KeyEventKind::Press, KeyCode::Up, _) => {
//             app.select_prev_field();
//             Ok(false)
//         }
//         (KeyEventKind::Press, KeyCode::Left, _) | (KeyEventKind::Press, KeyCode::Right, _) => {
//             app.tab_toggle();
//             Ok(false)
//         }
//         (KeyEventKind::Press, KeyCode::Char(code), _) => {
//             if code == 'q' {
//                 Ok(true)
//             } else if code == 'l' || code == 'h' {
//                 match app.tab_selected {
//                     Tabs::PomodoroSetting => app.tab_toggle(),
//                     Tabs::TaskManager => app.push_user_input_field(code),
//                 }
//                 Ok(false)
//             } else if code == 'j' {
//                 match app.tab_selected {
//                     Tabs::PomodoroSetting => app.select_next_field(),
//                     Tabs::TaskManager => app.push_user_input_field(code),
//                 }
//                 Ok(false)
//             } else if code == 'k' {
//                 match app.tab_selected {
//                     Tabs::PomodoroSetting => app.select_prev_field(),
//                     Tabs::TaskManager => app.push_user_input_field(code),
//                 }
//                 Ok(false)
//             } else if code == 'm' {
//                 match app.tab_selected {
//                     Tabs::PomodoroSetting => app.abort_timer(),
//                     Tabs::TaskManager => app.push_user_input_field(code),
//                 }
//                 Ok(false)
//             } else if code == 'p' {
//                 match app.tab_selected {
//                     Tabs::PomodoroSetting => app.abort_timer(),
//                     Tabs::TaskManager => app.push_user_input_field(code),
//                 }
//                 Ok(false)
//             } else if code == 'c' {
//                 match app.tab_selected {
//                     Tabs::PomodoroSetting => app.pause_timer(),
//                     Tabs::TaskManager => app.push_user_input_field(code),
//                 }
//                 Ok(false)
//             } else if code == 'w' {
//                 match app.tab_selected {
//                     Tabs::PomodoroSetting => app.task_list.previous_entry(),
//                     Tabs::TaskManager => app.push_user_input_field(code),
//                 }
//                 Ok(false)
//             } else if code == 's' {
//                 match app.tab_selected {
//                     Tabs::PomodoroSetting => app.task_list.next_entry(),
//                     Tabs::TaskManager => app.push_user_input_field(code),
//                 }
//                 Ok(false)
//             } else if code == 'x' {
//                 match app.tab_selected {
//                     Tabs::PomodoroSetting => app.delete_task(),
//                     Tabs::TaskManager => app.push_user_input_field(code),
//                 }
//                 Ok(false)
//             } else if code == ' ' {
//                 match app.tab_selected {
//                     Tabs::PomodoroSetting => app.toggle_timer(),
//                     Tabs::TaskManager => app.push_user_input_field(code),
//                 }
//                 Ok(false)
//             } else {
//                 Ok(false)
//             }
//         }
//         _ => Ok(false),
//     }
// }
