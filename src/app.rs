#![allow(clippy::single_match)]
#![allow(clippy::type_complexity)]

use std::time::{Duration, Instant};

const SECS_PER_MINUTE: u64 = 60;
const POMODORO_LENGTH: u64 = 25;
const SHORT_BREAK_LENGTH: u64 = 5;
const LONG_BREAK_LENGTH: u64 = 15;

/// State(whether the timer is up)
#[derive(Default)]
pub enum State {
    #[default]
    Pomodoro,
    ShortBreak,
    LongBreak,
}

#[derive(Default)]
pub struct App {
    // this field to store user settings
    pub timer_setting: TimerSetting,
    // this field is for echoing user input
    user_input: Input,
    // Running timer
    timer: Option<Timer>,
    // TODO The state of Pomodoro
    pub state: State,
    // this field let user choose tabs
    pub tab_selected: usize,
}

pub struct Timer {
    total_time: Duration,
    launch_timer: Instant,
    /// TODO for future statistics feature
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

pub struct Input {
    timer: String,
    short_break: String,
    long_break: String,
    field_selected: usize,
}

impl Input {
    pub fn display(&self) -> ((&str, &str), (&str, &str), (&str, &str)) {
        let (s1, s2, s3) = match self.field_selected {
            0 => (
                ">> Timer Length: ",
                "Short Break Length: ",
                "Long Break Length: ",
            ),
            1 => (
                "Timer Length: ",
                ">> Short Break Length: ",
                "Long Break Length: ",
            ),
            2 => (
                "Timer Length: ",
                "Short Break Length: ",
                ">> Long Break Length: ",
            ),
            _ => panic!("Not implemented Field"),
        };
        (
            (s1, self.timer.as_str()),
            (s2, self.short_break.as_str()),
            (s3, self.long_break.as_str()),
        )
    }

    pub fn get_field_mut(&mut self) -> &mut String {
        match self.field_selected {
            0 => &mut self.timer,
            1 => &mut self.short_break,
            2 => &mut self.long_break,
            _ => panic!("Not implemented input field"),
        }
    }

    pub fn select_prev_field(&mut self) {
        match self.field_selected {
            0 => self.field_selected = 2,
            1 => self.field_selected = 0,
            2 => self.field_selected = 1,
            _ => panic!("Not implemented input field"),
        }
    }
    pub fn select_next_field(&mut self) {
        match self.field_selected {
            0 => self.field_selected = 1,
            1 => self.field_selected = 2,
            2 => self.field_selected = 0,
            _ => panic!("Not implemented input field"),
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Self {
            timer: POMODORO_LENGTH.to_string(),
            short_break: SHORT_BREAK_LENGTH.to_string(),
            long_break: LONG_BREAK_LENGTH.to_string(),
            field_selected: 0,
        }
    }
}

pub struct TimerSetting {
    pub timer: Duration,
    pub short_break: Duration,
    pub long_break: Duration,
}

impl Default for TimerSetting {
    fn default() -> Self {
        TimerSetting {
            // timer: Duration::from_secs(SECS_PER_MINUTE * POMODORO_LENGTH),
            timer: Duration::from_secs(8),
            short_break: Duration::from_secs(SECS_PER_MINUTE * SHORT_BREAK_LENGTH),
            long_break: Duration::from_secs(SECS_PER_MINUTE * LONG_BREAK_LENGTH),
        }
    }
}

impl App {
    pub fn tab_toggle(&mut self) {
        match self.tab_selected {
            0 => self.tab_selected = 1,
            1 => self.tab_selected = 0,
            _ => panic!("Not implemented Tab"),
        }
    }

    pub fn clear_input_field(&mut self) {
        self.user_input.get_field_mut().clear();
    }

    pub fn select_next_field(&mut self) {
        self.user_input.select_next_field();
    }

    pub fn select_prev_field(&mut self) {
        self.user_input.select_prev_field();
    }

    pub fn push_user_input_field(&mut self, c: char) {
        self.user_input.get_field_mut().push(c);
    }

    pub fn pop_user_input_field(&mut self) {
        self.user_input.get_field_mut().pop();
    }

    pub fn display_user_input(&self) -> ((&str, &str), (&str, &str), (&str, &str)) {
        self.user_input.display()
    }

    pub fn abort_timer(&mut self) {
        match self.timer {
            Some(_) => {
                self.timer = None;
                // TODO go to next state
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
        }
    }

    pub fn launch_timer(&mut self) {
        match self.timer {
            None => {
                // TODO considering state
                self.timer = Some(Timer::new(self.timer_setting.timer));
            }
            Some(_) => {}
        }
    }

    pub fn update(&mut self) {
        if let Some(ref mut timer) = self.timer {
            if timer.is_finished() {
                // TODO after some short breaks, switch to long break
                match self.state {
                    State::Pomodoro => self.state = State::ShortBreak,
                    State::ShortBreak => self.state = State::Pomodoro,
                    State::LongBreak => self.state = State::Pomodoro,
                }
            } else {
                timer.update();
            }
        } // else the timer is not started, nothing to update
    }

    pub fn get_time_left(&self) -> u64 {
        match self.timer {
            Some(ref timer) => timer.get_time_left().as_secs(),
            None => {
                match self.state {
                    State::Pomodoro => self.timer_setting.timer.as_secs(),
                    State::ShortBreak => self.timer_setting.short_break.as_secs(),
                    State::LongBreak => self.timer_setting.long_break.as_secs()
                }
            }
        }
    }

    pub fn set_timer(&mut self) {
        match (
            self.user_input.timer.parse::<u64>(),
            self.user_input.short_break.parse::<u64>(),
            self.user_input.long_break.parse::<u64>(),
        ) {
            (Ok(timer), Ok(short_break), Ok(long_break)) => {
                self.timer_setting = TimerSetting {
                    timer: Duration::from_secs(timer * SECS_PER_MINUTE),
                    short_break: Duration::from_secs(short_break * SECS_PER_MINUTE),
                    long_break: Duration::from_secs(long_break * SECS_PER_MINUTE),
                };
            }
            _ => {
                self.user_input = Input::default();
                self.timer_setting = TimerSetting::default();
            }
        }
    }
}
