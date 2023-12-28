use crate::app::{AppAction, App};
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
use std::collections::HashMap;


lazy_static! {
    pub static ref KEYBINDINGS: HashMap<KeyEvent, AppAction> = {
    let mut m = HashMap::new();
    // NOTE function pointer in Rust works in this way
    // https://doc.rust-lang.org/std/primitive.fn.html#creating-function-pointers
    let toggle_timer: AppAction = App::toggle_timer;
    m.insert(
        KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE),
        toggle_timer,
    );
    m
    };
}
