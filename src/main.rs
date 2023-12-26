use core::panic;
// TODO Wrap and trim in ratatui
use std::{
    io::{self, stdout},
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

pub struct App {
    timer: Timer,
    start_time: Option<Instant>,
    user_input: Input,
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
        // TODO return saturated user input
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
            timer: String::from("25"),
            short_break: String::from("25"),
            long_break: String::from("25"),
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
            timer: Duration::from_secs(60 * 25),
            short_break: Duration::from_secs(60 * 25),
            long_break: Duration::from_secs(60 * 25),
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

    pub fn launch_timer(&mut self) {
        match self.start_time {
            Some(_) => {}
            None => {
                self.start_time = Some(Instant::now());
            }
        }
    }

    pub fn set_timer(&mut self) {
        self.timer.timer = Duration::from_secs(
            self.user_input
                .timer
                .parse::<u64>()
                .expect("Fail to parse input")
                * 60,
        );
        self.timer.short_break = Duration::from_secs(
            self.user_input
                .short_break
                .parse::<u64>()
                .expect("Fail to parse user input")
                * 60,
        );
        self.timer.long_break = Duration::from_secs(
            self.user_input
                .long_break
                .parse::<u64>()
                .expect("Fail to parse user input")
                * 60,
        );
    }
}

impl Default for App {
    fn default() -> Self {
        App {
            timer: Timer::default(),
            start_time: None,
            user_input: Input::default(),
            tab_selected: 0,
        }
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut should_quit = false;

    let mut app = App::default(); // initialize App

    while !should_quit {
        terminal.draw(|frame| ui(frame, &app))?;
        should_quit = handle_events(&mut app)?;
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
    let d1 = Block::default()
        .title("Usage")
        .borders(Borders::ALL)
        .style(Style::default());

    let layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)],
    )
    .split(area);

    frame.render_widget(d1, layout[0]);

    render_user_input_fields(frame, layout[1], app);
}

fn render_user_input_fields(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::new(
        Direction::Vertical,
        [Constraint::Length(1), Constraint::default()],
    )
    .split(area);
    // FIXME no tabs line displayed
    let tabs = Tabs::new::<Line>(vec!["t1".into(), "t2".into(), "t3".into()])
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tab_selected);
    frame.render_widget(tabs, chunks[0]);

    match app.tab_selected {
        0 => render_settings(frame, chunks[1], app),
        1 => render_task_manager(frame, chunks[1], app),
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
    // TODO highlight selected field
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
