#![allow(clippy::single_match)]
#![allow(clippy::type_complexity)]

use std::time::{Duration, Instant};

use crate::custom_widgets::StatefulList;

const SECS_PER_MINUTE: u64 = 60;
const DEFAULT_POMODORO_LENGTH: u64 = 25;
const DEFAULT_SHORT_BREAK_LENGTH: u64 = 5;
const DEFAULT_LONG_BREAK_LENGTH: u64 = 15;
const DEFAULT_POMODORO_PER_LONG_BREAK: u64 = 4;
const DEFAULT_POMODORO_PER_TASK: u64 = 1;

pub type AppAction = fn(&mut App);


#[derive(Debug)]
pub enum State {
    Pomodoro(u64),
    ShortBreak(u64),
    LongBreak,
}

impl Default for State {
    fn default() -> Self {
        if cfg!(debug_assertions) {
            Self::Pomodoro(2)
        } else {
            Self::Pomodoro(DEFAULT_POMODORO_PER_LONG_BREAK)
        }
    }
}

#[derive(Debug)]
pub struct App {
    // The Actual timer, None if timer is not running
    timer: Option<Timer>,
    // NOTE pomodoro current loop state
    pub state: State,
    pub tab_selected: Tabs,
    // NOTE Tab1: Pomodoro Settings DONE
    pub timer_setting: TimerSetting,
    pub timer_setting_input: Input, // NOTE TimerSetting input fields
    pub state_setting: StateSetting,
    // HACK use proc macro to generate input fields receiver struct and implementation
    pub task_manager_input: Input1, // NOTE TimerSetting input fields
    pub task_list: StatefulList<Task>,
    pub should_quit: bool,
    pub page_selected: Page,
}

#[derive(Default, Debug)]
pub enum Page {
    Help,
    #[default]
    Normal,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Tabs {
    #[default]
    PomodoroSetting,
    TaskManager,
}

impl Tabs {
    pub fn toggle(self) -> Self {
        match self {
            Tabs::TaskManager => Tabs::PomodoroSetting,
            Tabs::PomodoroSetting => Tabs::TaskManager,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        if cfg!(debug_assertions) {
            let task1 = Task {
                title: "Pomodoro Timer Dev".to_string(),
                notes: String::new(),
                pomodoros: 2,
            };
            let task2 = Task {
                title: "Renpy GalGame Dev".to_string(),
                notes: String::new(),
                pomodoros: 2,
            };
            let task_list = vec![Task::default(), task1, task2];
            let task_list = StatefulList::with_items(task_list);
            App {
                timer_setting: TimerSetting::default(),
                timer_setting_input: Input::default(),
                timer: None,
                state_setting: StateSetting::default(),
                state: State::default(),
                task_manager_input: Input1::default(),
                tab_selected: Tabs::default(),
                task_list,
                should_quit: false,
                page_selected: Page::default(),
            }
        } else {
            App {
                timer_setting: TimerSetting::default(),
                timer_setting_input: Input::default(),
                timer: None,
                state_setting: StateSetting::default(),
                state: State::default(),
                task_manager_input: Input1::default(),
                tab_selected: Tabs::default(),
                task_list: StatefulList::default(),
                should_quit: false,
                page_selected: Page::default(),
            }
        }
    }
}

#[derive(Debug)]
pub struct StateSetting {
    pomodoro_per_long_break: u64,
}

/// HACK task start time, end time for future statistics feature
#[derive(Debug, Clone)]
pub struct Task {
    title: String,
    pub notes: String,
    pub pomodoros: u64,
}

impl Default for Task {
    fn default() -> Self {
        let (title, notes) = if cfg!(debug_assertions) {
            (String::from("Unix Programming"), String::new())
        } else {
            (String::new(), String::new())
        };
        Self {
            title,
            notes,
            pomodoros: DEFAULT_POMODORO_PER_TASK,
        }
    }
}

impl Task {
    pub fn title(&self) -> &str {
        &self.title
    }
}

impl Default for StateSetting {
    fn default() -> Self {
        Self {
            pomodoro_per_long_break: DEFAULT_POMODORO_PER_LONG_BREAK,
        }
    }
}

#[derive(Debug)]
pub struct Timer {
    total_time: Duration,
    launch_timer: Instant,
    /// HACK for future statistics feature
    time_passed: Duration,
    pause_timer: Option<Instant>,
    time_pause: Duration,
}

impl Timer {
    pub fn new(time: Duration) -> Self {
        let now = Instant::now();
        Self {
            total_time: time,
            launch_timer: now,
            time_passed: Duration::ZERO,
            pause_timer: None,
            time_pause: Duration::ZERO,
        }
    }

    pub fn get_time_left(&self) -> Duration {
        self.total_time.saturating_sub(self.time_passed)
    }

    pub fn pause(&mut self) {
        match self.pause_timer {
            Some(_) => {}
            None => {
                self.pause_timer = Some(Instant::now());
            }
        }
    }

    pub fn resume(&mut self) {
        match self.pause_timer {
            Some(timer) => {
                self.time_pause += timer.elapsed();
                self.pause_timer = None;
            }
            None => {}
        }
    }

    pub fn is_finished(&self) -> bool {
        self.time_passed > self.total_time
    }

    pub fn update(&mut self) {
        if self.pause_timer.is_none() {
            self.time_passed = self.launch_timer.elapsed().saturating_sub(self.time_pause);
        }
    }

    pub fn is_paused(&self) -> bool {
        self.pause_timer.is_some()
    }
}

/// HACK generate corresponding input fields from settings needed
/// HACK Stateful List Like implementation
#[derive(Debug)]
pub struct Input {
    timer: String,
    short_break: String,
    long_break: String,
    pomodoro_per_long_break: String,
    field_selected: InputField,
}

#[derive(Clone, Copy, Debug)]
enum InputField {
    Timer,
    ShortBreak,
    LongBreak,
    PomodoroPerLongBreak,
}

#[derive(Debug, Default)]
pub struct Input1 {
    task_name: String,
    task_notes: String,
    pomodoros_per_task: String,
    field_selected: InputField1,
}

impl Input1 {
    pub fn display(&self) -> ((&str, &str), (&str, &str), (&str, &str)) {
        // HACK refactor this bullshit type
        let (s1, s2, s3) = match self.field_selected {
            InputField1::TaskName => (">> Task Name: ", "Est Pomodoros: ", "Task Notes:"),
            InputField1::PomodorosPerTask => ("Task Name: ", ">> Est Pomodoros: ", "Task Notes:"),
            InputField1::TaskNotes => ("Task Name: ", "Est Pomodoros: ", ">> Task Notes:"),
        };
        (
            (s1, &self.task_name),
            (s2, &self.pomodoros_per_task),
            (s3, &self.task_notes),
        )
    }

    pub fn get_field_mut(&mut self) -> &mut String {
        match self.field_selected {
            InputField1::TaskNotes => &mut self.task_notes,
            InputField1::PomodorosPerTask => &mut self.pomodoros_per_task,
            InputField1::TaskName => &mut self.task_name,
        }
    }

    pub fn select_prev_field(&mut self) {
        self.field_selected = self.field_selected.prev_field();
    }
    pub fn select_next_field(&mut self) {
        self.field_selected = self.field_selected.next_field();
    }
}

#[derive(Clone, Copy, Debug, Default)]
enum InputField1 {
    #[default]
    TaskName,
    TaskNotes,
    PomodorosPerTask,
}

impl InputField1 {
    pub fn next_field(self) -> Self {
        match self {
            InputField1::TaskName => InputField1::PomodorosPerTask,
            InputField1::PomodorosPerTask => InputField1::TaskNotes,
            InputField1::TaskNotes => InputField1::TaskName,
        }
    }
    pub fn prev_field(self) -> Self {
        match self {
            InputField1::TaskName => InputField1::TaskNotes,
            InputField1::PomodorosPerTask => InputField1::TaskName,
            InputField1::TaskNotes => InputField1::PomodorosPerTask,
        }
    }
}

impl InputField {
    pub fn prev_field(self) -> Self {
        match self {
            InputField::Timer => InputField::PomodoroPerLongBreak,
            InputField::ShortBreak => InputField::Timer,
            InputField::LongBreak => InputField::ShortBreak,
            InputField::PomodoroPerLongBreak => InputField::LongBreak,
        }
    }
    pub fn next_field(self) -> Self {
        match self {
            InputField::Timer => InputField::ShortBreak,
            InputField::ShortBreak => InputField::LongBreak,
            InputField::LongBreak => InputField::PomodoroPerLongBreak,
            InputField::PomodoroPerLongBreak => InputField::Timer,
        }
    }
}

impl Input {
    // HACK refactor this bullshit type
    pub fn display(&self) -> ((&str, &str), (&str, &str), (&str, &str), (&str, &str)) {
        let (s1, s2, s3, s4) = match self.field_selected {
            InputField::Timer => (
                ">> Timer Length: ",
                "Short Break Length: ",
                "Long Break Length: ",
                "Pomodoros Per Long Break: ",
            ),
            InputField::ShortBreak => (
                "Timer Length: ",
                ">> Short Break Length: ",
                "Long Break Length: ",
                "Pomodoros Per Long Break: ",
            ),
            InputField::LongBreak => (
                "Timer Length: ",
                "Short Break Length: ",
                ">> Long Break Length: ",
                "Pomodoros Per Long Break: ",
            ),
            InputField::PomodoroPerLongBreak => (
                "Timer Length: ",
                "Short Break Length: ",
                "Long Break Length: ",
                ">> Pomodoros Per Long Break: ",
            ),
        };
        (
            (s1, self.timer.as_str()),
            (s2, self.short_break.as_str()),
            (s3, self.long_break.as_str()),
            (s4, self.pomodoro_per_long_break.as_str()),
        )
    }

    pub fn get_field_mut(&mut self) -> &mut String {
        match self.field_selected {
            InputField::Timer => &mut self.timer,
            InputField::ShortBreak => &mut self.short_break,
            InputField::LongBreak => &mut self.long_break,
            InputField::PomodoroPerLongBreak => &mut self.pomodoro_per_long_break,
        }
    }

    pub fn select_prev_field(&mut self) {
        self.field_selected = self.field_selected.prev_field();
    }
    pub fn select_next_field(&mut self) {
        self.field_selected = self.field_selected.next_field();
    }
}

impl Default for Input {
    fn default() -> Self {
        Self {
            timer: DEFAULT_POMODORO_LENGTH.to_string(),
            short_break: DEFAULT_SHORT_BREAK_LENGTH.to_string(),
            long_break: DEFAULT_LONG_BREAK_LENGTH.to_string(),
            pomodoro_per_long_break: DEFAULT_POMODORO_PER_LONG_BREAK.to_string(),
            field_selected: InputField::Timer,
        }
    }
}

#[derive(Debug)]
pub struct TimerSetting {
    pub timer: Duration,
    pub short_break: Duration,
    pub long_break: Duration,
}

impl Default for TimerSetting {
    fn default() -> Self {
        if cfg!(debug_assertions) {
            TimerSetting {
                timer: Duration::from_secs(9),
                short_break: Duration::from_secs(1),
                long_break: Duration::from_secs(2),
            }
        } else {
            TimerSetting {
                timer: Duration::from_secs(SECS_PER_MINUTE * DEFAULT_POMODORO_LENGTH),
                short_break: Duration::from_secs(SECS_PER_MINUTE * DEFAULT_SHORT_BREAK_LENGTH),
                long_break: Duration::from_secs(SECS_PER_MINUTE * DEFAULT_LONG_BREAK_LENGTH),
            }
        }
    }
}

impl App {
    pub fn tab_toggle(&mut self) {
        self.tab_selected = self.tab_selected.toggle();
    }

    pub fn clear_input_field(&mut self) {
        match self.tab_selected {
            Tabs::TaskManager => self.task_manager_input.get_field_mut().clear(),
            Tabs::PomodoroSetting => self.timer_setting_input.get_field_mut().clear(),
        }
        self.timer_setting_input.get_field_mut().clear();
    }

    pub fn select_next_field(&mut self) {
        match self.tab_selected {
            Tabs::TaskManager => self.task_manager_input.select_next_field(),
            Tabs::PomodoroSetting => self.timer_setting_input.select_next_field(),
        }
    }

    pub fn select_prev_field(&mut self) {
        match self.tab_selected {
            Tabs::TaskManager => self.task_manager_input.select_prev_field(),
            Tabs::PomodoroSetting => self.timer_setting_input.select_prev_field(),
        }
    }

    pub fn push_user_input_field(&mut self, c: char) {
        match self.tab_selected {
            Tabs::TaskManager => self.task_manager_input.get_field_mut().push(c),
            Tabs::PomodoroSetting => self.timer_setting_input.get_field_mut().push(c),
        }
    }

    pub fn pop_user_input_field(&mut self) {
        match self.tab_selected {
            Tabs::TaskManager => self.task_manager_input.get_field_mut().pop(),
            Tabs::PomodoroSetting => self.timer_setting_input.get_field_mut().pop(),
        };
    }

    pub fn abort_timer(&mut self) {
        match self.timer {
            Some(_) => {
                self.timer = None;
                match self.state {
                    State::Pomodoro(0) => self.state = State::LongBreak,
                    State::Pomodoro(x) => self.state = State::ShortBreak(x - 1),
                    State::ShortBreak(x) => self.state = State::Pomodoro(x),
                    State::LongBreak => {
                        self.state = State::Pomodoro(self.state_setting.pomodoro_per_long_break)
                    }
                };
            }
            None => {}
        };
    }

    pub fn toggle_timer(&mut self) {
        match self.timer {
            Some(ref mut timer) => {
                if timer.is_paused() {
                    timer.resume();
                } else {
                    timer.pause();
                }
            }
            None => self.launch_timer(),
        }
    }

    pub fn pause_timer(&mut self) {
        if let Some(ref mut timer) = self.timer {
            timer.pause();
        } // do nothing when no timer is running
    }

    pub fn launch_timer(&mut self) {
        match self.timer {
            None => {
                let time = match self.state {
                    State::Pomodoro(_) => self.timer_setting.timer,
                    State::LongBreak => self.timer_setting.long_break,
                    State::ShortBreak(_) => self.timer_setting.short_break,
                };
                self.timer = Some(Timer::new(time));
            }
            // there is a timer running, we do nothing
            Some(_) => {}
        }
    }

    pub fn update(&mut self) {
        if let Some(ref mut timer) = self.timer {
            if timer.is_finished() {
                match self.state {
                    State::Pomodoro(0) => self.state = State::LongBreak,
                    State::Pomodoro(x) => self.state = State::ShortBreak(x - 1),
                    State::ShortBreak(x) => self.state = State::Pomodoro(x),
                    State::LongBreak => {
                        self.state = State::Pomodoro(self.state_setting.pomodoro_per_long_break)
                    }
                }
                // When time is up, we set timer back to None
                self.timer = None;
            } else {
                timer.update();
            }
        } // else the timer is not started, nothing to update
    }

    pub fn get_time_left(&self) -> u64 {
        match self.timer {
            Some(ref timer) => timer.get_time_left().as_secs(),
            None => match self.state {
                State::Pomodoro(_) => self.timer_setting.timer.as_secs(),
                State::ShortBreak(_) => self.timer_setting.short_break.as_secs(),
                State::LongBreak => self.timer_setting.long_break.as_secs(),
            },
        }
    }

    pub fn set_timer(&mut self) {
        match (
            self.timer_setting_input.timer.parse::<u64>(),
            self.timer_setting_input.short_break.parse::<u64>(),
            self.timer_setting_input.long_break.parse::<u64>(),
            self.timer_setting_input
                .pomodoro_per_long_break
                .parse::<u64>(),
        ) {
            (Ok(mut timer), Ok(mut short_break), Ok(mut long_break), Ok(pomodoro_per_long_break)) => {
                timer = if timer < 100 { timer }  else { DEFAULT_POMODORO_LENGTH };
                short_break = if short_break < 100 { short_break }  else { DEFAULT_SHORT_BREAK_LENGTH };
                long_break = if long_break < 100 { long_break }  else { DEFAULT_LONG_BREAK_LENGTH };
                self.timer_setting = TimerSetting {
                    timer: Duration::from_secs(timer * SECS_PER_MINUTE),
                    short_break: Duration::from_secs(short_break * SECS_PER_MINUTE),
                    long_break: Duration::from_secs(long_break * SECS_PER_MINUTE),
                };
                // HACK we can let user choose here
                // it is a thing that when you change this value when the pomodoro loop has
                // started; the change will apply next loop
                // because we don't modify the value of self.state to change its behavior
                self.state_setting = StateSetting {
                    pomodoro_per_long_break,
                };
            }
            _ => {
                self.timer_setting_input = Input::default();
                self.timer_setting = TimerSetting::default();
                self.state_setting = StateSetting::default();
            }
        }
    }

    pub fn add_task(&mut self) {
        // TODO more robust add task
        let task = Task {
            title: self.task_manager_input.task_name.clone(),
            notes: self.task_manager_input.task_notes.clone(),
            pomodoros: self
                .task_manager_input
                .pomodoros_per_task
                .parse()
                .unwrap_or(1),
        };
        self.task_list.items.push(task);
    }

    pub fn delete_task(&mut self) {
        match self.task_list.state.selected() {
            None => {}
            Some(idx) => {
                self.task_list.items.remove(idx);
                // FIXME reasonable deletion
                self.task_list.previous_entry();
            }
        };
    }

    pub fn quit(&mut self) {
        match self.page_selected {
            Page::Help => self.page_selected = Page::default(),
            Page::Normal => self.should_quit = true,
        }
    }

    pub fn open_help(&mut self) {
        self.page_selected = Page::Help;
    }
}
