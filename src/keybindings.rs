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

const OPEN_HELP: AppAction = App::open_help;

const QUIT_APP: AppAction = App::quit;

lazy_static! {
    pub static ref TIMER_SETTING_KEYBINDINGS: HashMap<KeyEvent, AppAction> = {
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
            QUIT_APP,
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
    // open help
        m.insert(
            KeyEvent::new(KeyCode::Char('?'), KeyModifiers::NONE),
            OPEN_HELP,
        );
        m
    };

    /// NOTE keybinding for TaskManager
    pub static ref TASK_MANAGER_KEYBINDINGS: HashMap<KeyEvent, AppAction> = {
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
    // TODO generate Keybindings Help from this static Hashmap
    pub static ref KEYBINDINGS_HELP_MESSAGE: HashMap<AppAction, &'static str> = {
        let mut m = HashMap::new();
        m.insert(TAB_TOGGLE, "Switch to next Tab");

        m.insert(CLEAR_INPUT_FIELD, "Clear current input field");

        m.insert(SELECT_NEXT_FIELD, "Select next input field");

        m.insert(SELECT_PREV_FIELD, "Select previous input field");

        m.insert(POP_USER_INPUT_FIELD, "Delete the last character in input field");

        m.insert(ABORT_TIMER, "Skip timer and go to next stage");

        m.insert(TOGGLE_TIMER, "Launch or Pause the timer");

        m.insert(PAUSE_TIMER, "Pause the timer");

        m.insert(LAUNCH_TIMER, "Launch the timer");

        m.insert(SET_TIMER, "Confirm and change current timer setting");

        m.insert(ADD_TASK, "Confirm and add task to task list");

        m.insert(DELETE_TASK, "Delete current selected task");

        m.insert(OPEN_HELP, "Open this help page");
        m
    };
}
