use std::time::{Duration, Instant};

const SECS_PER_MINUTE: u64 = 60;
const POMODORO_LENGTH: u64 = 25;
const SHORT_BREAK_LENGTH: u64 = 5;
const LONG_BREAK_LENGTH: u64 = 15;
/// State(whether the timer is up)
pub enum State {
    Pomodoro,
    ShortBreak,
    LongBreak,
}

impl Default for State {
    fn default() -> Self {
        State::Pomodoro
    }
}

#[derive(Default)]
pub struct App {
    // this field to store user settings
    timer_setting: TimerSetting,
    // this field is for echoing user input
    user_input: Input,
    // Running timer
    timer: Option<Timer>,
    // TODO The state of Pomodoro
    state: State,
    // this field let user choose tabs
    pub tab_selected: usize,
}

pub struct Timer {
    start_time: Instant,
    time_left: Duration,
    actual_start_time: Instant,
    actual_time_left: Duration,
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
    timer: Duration,
    short_break: Duration,
    long_break: Duration,
}

impl Default for TimerSetting {
    fn default() -> Self {
        TimerSetting {
            timer: Duration::from_secs(SECS_PER_MINUTE * POMODORO_LENGTH),
            // timer: Duration::from_secs(3),
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

    pub fn push_user_input_field(&mut self, c: char){
        self.user_input.get_field_mut().push(c);
    }

    pub fn pop_user_input_field(&mut self){
        self.user_input.get_field_mut().pop();
    }

    pub fn display_user_input(&self) -> ((&str, &str), (&str, &str), (&str, &str)){
        self.user_input.display()
    }

    pub fn pause_timer(&mut self) {
        todo!()
    }

    pub fn launch_timer(&mut self) {
        todo!()
    }

    pub fn update_timer(&mut self) {
        todo!()
    }

    pub fn set_timer(&mut self) {
        // FIXME when user input is greater than 100
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
