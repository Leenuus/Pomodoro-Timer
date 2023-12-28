#![allow(clippy::single_match)]
#![allow(clippy::type_complexity)]

use std::{
    default,
    time::{Duration, Instant},
};

use ratatui::widgets::ListState;

const SECS_PER_MINUTE: u64 = 60;
const POMODORO_LENGTH: u64 = 25;
const SHORT_BREAK_LENGTH: u64 = 5;
const LONG_BREAK_LENGTH: u64 = 15;
const DEFAULT_POMODORO_PER_LONG_BREAK: u64 = 4;
const DEFAULT_POMODORO_PER_TASK: u64 = 1;

#[derive(Debug, Default)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        if items.len() != 0 {
            StatefulList {
                state: ListState::default().with_selected(Some(0)),
                items,
            }
        } else {
            StatefulList {
                state: ListState::default(),
                items,
            }
        }
    }

    pub fn next_entry(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            // select first display item but not the first item
            None => self.state.offset(),
        };
        self.state.select(Some(i));
    }

    pub fn previous_entry(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.state.offset(),
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

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
    timer_setting_input: Input, // NOTE TimerSetting input fields
    pub state_setting: StateSetting,
    // TODO Tab2: Task Manager
    task_manager_input: Input, // NOTE TimerSetting input fields
    pub task_list: StatefulList<Task>,
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
            let task_list = vec![Task::default(); 3];
            let task_list = StatefulList::with_items(task_list);
            App {
                timer_setting: TimerSetting::default(),
                timer_setting_input: Input::default(),
                timer: None,
                state_setting: StateSetting::default(),
                state: State::default(),
                task_manager_input: Input::default(),
                tab_selected: Tabs::default(),
                task_list,
            }
        } else {
            App {
                timer_setting: TimerSetting::default(),
                timer_setting_input: Input::default(),
                timer: None,
                state_setting: StateSetting::default(),
                state: State::default(),
                task_manager_input: Input::default(),
                tab_selected: Tabs::default(),
                task_list: StatefulList::default(),
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
    // TODO refactor this bullshit type
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
            timer: POMODORO_LENGTH.to_string(),
            short_break: SHORT_BREAK_LENGTH.to_string(),
            long_break: LONG_BREAK_LENGTH.to_string(),
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
                timer: Duration::from_secs(3),
                short_break: Duration::from_secs(1),
                long_break: Duration::from_secs(2),
            }
        } else {
            TimerSetting {
                timer: Duration::from_secs(SECS_PER_MINUTE * POMODORO_LENGTH),
                short_break: Duration::from_secs(SECS_PER_MINUTE * SHORT_BREAK_LENGTH),
                long_break: Duration::from_secs(SECS_PER_MINUTE * LONG_BREAK_LENGTH),
            }
        }
    }
}

impl App {
    pub fn tab_toggle(&mut self) {
        self.tab_selected = self.tab_selected.toggle();
    }

    pub fn clear_input_field(&mut self) {
        self.timer_setting_input.get_field_mut().clear();
    }

    pub fn select_next_field(&mut self) {
        self.timer_setting_input.select_next_field();
    }

    pub fn select_prev_field(&mut self) {
        self.timer_setting_input.select_prev_field();
    }

    pub fn push_user_input_field(&mut self, c: char) {
        self.timer_setting_input.get_field_mut().push(c);
    }

    pub fn pop_user_input_field(&mut self) {
        self.timer_setting_input.get_field_mut().pop();
    }

    pub fn display_user_input(&self) -> ((&str, &str), (&str, &str), (&str, &str), (&str, &str)) {
        self.timer_setting_input.display()
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

    // FIXME tell user not to set a timer more than 100 minutes
    pub fn set_timer(&mut self) {
        match (
            self.timer_setting_input.timer.parse::<u64>(),
            self.timer_setting_input.short_break.parse::<u64>(),
            self.timer_setting_input.long_break.parse::<u64>(),
            self.timer_setting_input
                .pomodoro_per_long_break
                .parse::<u64>(),
        ) {
            (Ok(timer), Ok(short_break), Ok(long_break), Ok(pomodoro_per_long_break)) => {
                self.timer_setting = TimerSetting {
                    timer: Duration::from_secs(timer * SECS_PER_MINUTE),
                    short_break: Duration::from_secs(short_break * SECS_PER_MINUTE),
                    long_break: Duration::from_secs(long_break * SECS_PER_MINUTE),
                };
                // it is a thing that when you change this value when the pomodoro loop has
                // started; the change will apply next loop
                // because we don't modify the value of self.state to change its behavior
                // TODO we can let user choose here
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
}
