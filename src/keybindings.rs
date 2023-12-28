use crate::app::{App, AppAction};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;

// NOTE function pointer in Rust works in this way
// https://doc.rust-lang.org/std/primitive.fn.html#creating-function-pointers
const TAB_TOGGLE: AppAction = App::tab_toggle;

const CLEAR_INPUT_FIELD: AppAction = App::clear_input_field;

const SELECT_NEXT_FIELD: AppAction = App::select_next_field;

const SELECT_PREV_FIELD: AppAction = App::select_prev_field;

const POP_USER_INPUT_FIELD: AppAction = App::pop_user_input_field;

const ABORT_TIMER: AppAction = App::abort_timer;

const TOGGLE_TIMER: AppAction = App::toggle_timer;

const PAUSE_TIMER: AppAction = App::pause_timer;

const LAUNCH_TIMER: AppAction = App::launch_timer;

const SET_TIMER: AppAction = App::set_timer;

// TODO Task Manager Mode Keymap
const ADD_TASK: AppAction = App::add_task;

// TODO Task Manager Mode Keymap
const DELETE_TASK: AppAction = App::delete_task;

const QUIT_APP: AppAction = App::quit;

lazy_static! {
    pub static ref TimerSettingKeybindings: HashMap<KeyEvent, AppAction> = {
    let mut m = HashMap::new();
    m.insert(
        KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
        CLEAR_INPUT_FIELD,
    );
    m.insert(
        KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE),
        TAB_TOGGLE,
    );
    m.insert(
        KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE),
        TAB_TOGGLE,
    );
    m.insert(
        KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
        SELECT_NEXT_FIELD,
    );
    m.insert(
        KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE),
        SELECT_PREV_FIELD,
    );
    m.insert(
        KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE),
        POP_USER_INPUT_FIELD,
    );
    m.insert(
        KeyEvent::new(KeyCode::Char('Q'), KeyModifiers::NONE),
        QUIT_APP,
    );
    m.insert(
        KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
        ABORT_TIMER,
    );
    m.insert(
        KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE),
        TOGGLE_TIMER,
    );
    m.insert(
        KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE),
        PAUSE_TIMER,
    );
    m.insert(
        KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE),
        LAUNCH_TIMER,
    );
    m.insert(
        KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
        SET_TIMER,
    );

    m
    };
}
