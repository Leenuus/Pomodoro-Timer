use crate::app::{App, AppAction};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;

// NOTE function pointer in Rust works this way
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

const ADD_TASK: AppAction = App::add_task;

// TODO Task Manager Mode Keymap
const DELETE_TASK: AppAction = App::delete_task;

const QUIT_APP: AppAction = App::quit;

lazy_static! {
    pub static ref TimerSettingKeybindings: HashMap<KeyEvent, AppAction> = {
        let mut m = HashMap::new();
    // clear input field
        m.insert(
            KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            CLEAR_INPUT_FIELD,
        );
    // switch to another tab
        m.insert(
            KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE),
            TAB_TOGGLE,
        );
        m.insert(
            KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE),
            TAB_TOGGLE,
        );
        m.insert(
            KeyEvent::new(KeyCode::Left, KeyModifiers::NONE),
            TAB_TOGGLE,
        );
        m.insert(
            KeyEvent::new(KeyCode::Right, KeyModifiers::NONE),
            TAB_TOGGLE,
        );

    // select next field
        m.insert(
            KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
            SELECT_NEXT_FIELD,
        );
        m.insert(
            KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE),
            SELECT_NEXT_FIELD,
        );
        m.insert(
            KeyEvent::new(KeyCode::Down, KeyModifiers::NONE),
            SELECT_NEXT_FIELD,
        );
    // select previous field
        m.insert(
            KeyEvent::new(KeyCode::Up, KeyModifiers::NONE),
            SELECT_PREV_FIELD,
        );
        m.insert(
            KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE),
            SELECT_PREV_FIELD,
        );
        m.insert(
            KeyEvent::new(KeyCode::BackTab, KeyModifiers::NONE),
            SELECT_PREV_FIELD,
        );
    // backspace erase one char
        m.insert(
            KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE),
            POP_USER_INPUT_FIELD,
        );
    // quit app
        m.insert(
            KeyEvent::new(KeyCode::Char('Q'), KeyModifiers::NONE),
            QUIT_APP,
        );
    // abort timer
        m.insert(
            KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
            ABORT_TIMER,
        );
    // toggle timer state
        m.insert(
            KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE),
            TOGGLE_TIMER,
        );
    // pause timer
        m.insert(
            KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE),
            PAUSE_TIMER,
        );
    // launch timer
        m.insert(
            KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE),
            LAUNCH_TIMER,
        );
    // change pomodoro timer settings
        m.insert(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE), SET_TIMER);

        m
    };

    /// NOTE keybinding for TaskManager
    pub static ref TaskManagerKeybindings: HashMap<KeyEvent, AppAction> = {
        let mut m = HashMap::new();
    // clear field
        m.insert(
            KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            CLEAR_INPUT_FIELD,
    // switch tab
        );
        m.insert(
            KeyEvent::new(KeyCode::Left, KeyModifiers::NONE),
            TAB_TOGGLE,
        );
        m.insert(
            KeyEvent::new(KeyCode::Right, KeyModifiers::NONE),
            TAB_TOGGLE,
        );
    // select next field
        m.insert(
            KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE),
            SELECT_NEXT_FIELD,
        );
        m.insert(
            KeyEvent::new(KeyCode::Down, KeyModifiers::NONE),
            SELECT_NEXT_FIELD,
        );
    // select prev field
        m.insert(
            KeyEvent::new(KeyCode::BackTab, KeyModifiers::NONE),
            SELECT_PREV_FIELD,
        );
        m.insert(
            KeyEvent::new(KeyCode::Up, KeyModifiers::NONE),
            SELECT_PREV_FIELD,
        );
        m.insert(
            KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE),
            POP_USER_INPUT_FIELD,
        );
        m.insert(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE), ADD_TASK);
        m
    };
}
