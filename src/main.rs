use core::panic;
// TODO Wrap and trim in ratatui
use std::{
    io::{self, stdout},
    thread::sleep,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

pub mod digits_clock;
use crate::digits_clock::*;

const SECS_PER_MINUTE: u64 = 60;
const POMODORO_LENGTH: u64 = 25;
const SHORT_BREAK_LENGTH: u64 = 5;
const LONG_BREAK_LENGTH: u64 = 15;
const FPS: u64 = 30;

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
    timer: Timer,
    // this field is for echoing user input
    user_input: Input,
    //  these 3 fields are for keeping track of the running timer state
    start_time: Option<Instant>,
    time_left: Option<Duration>,
    state: State,
    // this field let user choose tabs
    tab_selected: usize,
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

pub struct Timer {
    timer: Duration,
    short_break: Duration,
    long_break: Duration,
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
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

    pub fn pause_timer(&mut self) {
        todo!()
    }

    // FIXME strange overflow
    pub fn launch_timer(&mut self) {
        let time = match self.state {
            State::Pomodoro => self.timer.timer,
            State::LongBreak => self.timer.long_break,
            State::ShortBreak => self.timer.short_break,
        };
        match (self.start_time, self.time_left) {
            (Some(start_time), Some(time_left)) => {
                // if the timer is already running
                // we update `time_left`
                // the timer is already running
                // FIXME sub overflow
                self.time_left = Some(time_left - start_time.elapsed());
            }
            (None, None) => {
                // if the timer is not up, then we launch it
                // we record the uptime and set time_left to timer's duration
                // the timer is not running before
                self.start_time = Some(Instant::now());
                self.time_left = Some(time);
            }
            (None, Some(_)) => {
                // when we pause the timer
                // we set the start_time back to None,
                // compute `time_left` and store it
                // when we launch from pause state again, we just create a new start_time
                self.start_time = Some(Instant::now());
            }
            _ => unreachable!("Bad logic if this is reached"),
        }
    }

    pub fn update_timer(&mut self) {
        match self.state {
            // FIXME after finishing one timer
            // `start_time` becomes Some(_) and never get set back
            _ => {}
        }
    }

    pub fn set_timer(&mut self) {
        // FIXME when user input length is ZERO
        // Set timer and user input back to default state
        match (
            self.user_input.timer.parse::<u64>(),
            self.user_input.short_break.parse::<u64>(),
            self.user_input.long_break.parse::<u64>(),
        ) {
            (Ok(timer), Ok(short_break), Ok(long_break)) => {
                self.timer = Timer {
                    timer: Duration::from_secs(timer * SECS_PER_MINUTE),
                    short_break: Duration::from_secs(short_break * SECS_PER_MINUTE),
                    long_break: Duration::from_secs(long_break * SECS_PER_MINUTE),
                };
            }
            _ => {
                self.user_input = Input::default();
                self.timer = Timer::default();
            }
        }
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut should_quit = false;
    const INTERVAL: u64 = 1000 / FPS;

    let mut app = App::default(); // initialize App

    while !should_quit {
        terminal.draw(|frame| ui(frame, &app))?;
        should_quit = handle_events(&mut app)?;
        sleep(Duration::from_millis(INTERVAL));
        // TODO handle timer here
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events(app: &mut App) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        match event::read()? {
            Event::Key(key) => {
                return handle_key(key, app);
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

// TODO vim like keybindings
fn handle_key(key: KeyEvent, app: &mut App) -> io::Result<bool> {
    match (key.kind, key.code, key.modifiers) {
        // we filter user illegal input here
        (KeyEventKind::Press, KeyCode::Char(code), _) if code as u8 >= 48 && code as u8 <= 57 => {
            app.user_input.get_field_mut().push(code);
            Ok(false)
        }
        (KeyEventKind::Press, KeyCode::Esc, _) => {
            app.user_input.get_field_mut().clear();
            Ok(false)
        }
        (KeyEventKind::Press, KeyCode::Backspace, _) => {
            app.user_input.get_field_mut().pop();
            Ok(false)
        }
        (KeyEventKind::Press, KeyCode::Enter, _) => {
            // TODO modify corresponding field
            app.set_timer();
            Ok(false)
        }
        (KeyEventKind::Press, KeyCode::Tab, _) => {
            app.user_input.select_next_field();
            Ok(false)
        }
        (KeyEventKind::Press, KeyCode::BackTab, _) => {
            // FIXME shift tab not work
            app.user_input.select_prev_field();
            Ok(false)
        }
        (KeyEventKind::Press, KeyCode::Char(code), _) => {
            if code == 'q' {
                Ok(true)
            } else if code == 'l' || code == 'h' {
                app.tab_toggle();
                Ok(false)
            } else if code == 'j' {
                app.user_input.select_next_field();
                Ok(false)
            } else if code == 'k' {
                app.user_input.select_prev_field();
                Ok(false)
            } else if code == ' ' {
                app.launch_timer();
                Ok(false)
            } else {
                Ok(false)
            }
        }
        _ => Ok(false),
    }
}

fn ui(frame: &mut Frame, app: &App) {
    let layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Ratio(1, 4), Constraint::Ratio(3, 4)],
    )
    .split(frame.size());
    render_task_list(frame, layout[0]);
    render_right_side(frame, layout[1], app);
}

fn render_task_list(frame: &mut Frame, area: Rect) {
    // TODO TASK List widget
    let items = ["Item 1", "Item 2", "Item 3"];
    let list = List::new(items)
        .block(Block::default().title("Task List").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::BottomToTop);

    frame.render_widget(list, area);
}

fn render_right_side(frame: &mut Frame, area: Rect, app: &App) {
    let rs = Layout::new(
        Direction::Vertical,
        [Constraint::Ratio(3, 4), Constraint::Ratio(1, 4)],
    )
    .split(area);

    render_digit_clock(frame, rs[0], app);

    render_console(frame, rs[1], app);
}

fn render_console(frame: &mut Frame, area: Rect, app: &App) {
    let layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)],
    )
    .split(area);

    render_usage_prompt(frame, layout[0]);
    render_user_input_fields(frame, layout[1], app);
}

fn render_usage_prompt(frame: &mut Frame, area: Rect) {
    // TODO usage prompt widget: render it with dynamic keymaps
    // we can generate usage help from keymap
    let d1 = Block::default()
        .title("Usage")
        .borders(Borders::ALL)
        .style(Style::default());
    frame.render_widget(d1, area);
}

fn render_user_input_fields(frame: &mut Frame, area: Rect, app: &App) {
    match app.tab_selected {
        0 => render_settings(frame, area, app),
        1 => render_task_manager(frame, area, app),
        _ => panic!("Tab selected index, not implemented"),
    };
}

fn render_task_manager(frame: &mut Frame, area: Rect, app: &App) {
    // TODO input field
    // TODO task name
    let b = Block::default()
        .title(app.tab_selected.to_string())
        .borders(Borders::LEFT | Borders::RIGHT)
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black));
    frame.render_widget(b, area);
}

fn render_settings(frame: &mut Frame, area: Rect, app: &App) {
    let ((s1, timer), (s2, short_break), (s3, long_break)) = app.user_input.display();
    // TODO Add some padding to this widget
    let text = vec![
        Line::from(vec![
            Span::styled(s1, Style::new().green().italic()),
            Span::from(timer).style(Style::default()),
            Span::styled("  min", Style::new().blue().italic()),
        ]),
        Line::default(),
        Line::from(vec![
            Span::styled(s2, Style::new().green().italic()),
            Span::from(short_break).style(Style::default()),
            Span::styled("  min", Style::new().blue().italic()),
        ]),
        Line::default(),
        Line::from(vec![
            Span::styled(s3, Style::new().green().italic()),
            Span::from(long_break).style(Style::default()),
            Span::styled("  min", Style::new().blue().italic()),
        ]),
    ];
    let b = Paragraph::new(text)
        .block(Block::new().title("Settings").borders(Borders::ALL))
        .style(Style::default())
        .alignment(Alignment::Left);
    // .wrap(Wrap { trim: true });
    frame.render_widget(b, area);
}
