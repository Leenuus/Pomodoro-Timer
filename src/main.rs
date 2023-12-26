use std::{
    io::{self, stdout},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

pub mod digits;
use crate::digits::*;


struct App {
    timer: Duration,
    start_time: Instant,
    user_input: Vec<char>,
    tab_selected: usize,
}

impl App {
    pub fn tab_toggle(&mut self) {
        match self.tab_selected{
            0 => self.tab_selected = 1,
            1 => self.tab_selected = 0,
            _ => panic!("Not implemented Tab")
        }
    }
}

impl Default for App {
    fn default() -> Self {
        App {
            timer: Duration::from_secs(60 * 25),
            start_time: Instant::now(),
            user_input: vec![],
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
    //  TODO get user input to set the timer
    // let buf: Vec<char> = Vec::new();
    if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
        return Ok(true);
    }
    if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('h') {
        app.tab_toggle();
        return Ok(false);
    }
    if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('l') {
        app.tab_toggle();
        return Ok(false);
    }
    Ok(false)
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

// TODO not only timer, also __stop watch__
// TODO Yet one more feature, different style of clock using `tab` feature
fn render_digit_clock(frame: &mut Frame, area: Rect, app: &App) {
    let layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
        ],
    )
    .split(area);

    let time_left = app.timer.saturating_sub(app.start_time.elapsed()).as_secs();
    let (d1, d2, d3, d4) = time_convert(time_left);
    render_clock_digit(frame, layout[0], d1, 0);

    render_clock_digit(frame, layout[1], d2, 1);

    render_clock_digit(frame, layout[2], d3, 2);

    render_clock_digit(frame, layout[3], d4, 3);
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

fn render_clock_digit(frame: &mut Frame, layout: Rect, digit: &str, index: u8) {
    let (borders_top, borders_bottom, borders_middle) = match index {
        0 => (
            Borders::TOP | Borders::LEFT,
            Borders::BOTTOM | Borders::LEFT,
            Borders::LEFT,
        ),
        x if x == 1 || x == 2 => (Borders::TOP, Borders::BOTTOM, Borders::NONE),

        3 => (
            Borders::TOP | Borders::RIGHT,
            Borders::BOTTOM | Borders::RIGHT,
            Borders::RIGHT,
        ),
        _ => {
            panic!()
        }
    };

    let d1 = render_digit(digit, borders_middle);

    let layout1 = Layout::new(
        Direction::Vertical,
        [
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ],
    )
    .split(layout);
    let b1 = Block::default()
        .borders(borders_top)
        .style(Style::default());
    frame.render_widget(b1, layout1[0]);
    frame.render_widget(d1, layout1[1]);
    let b2 = Block::default()
        .borders(borders_bottom)
        .style(Style::default());
    frame.render_widget(b2, layout1[2]);
}


fn render_user_input_fields(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::new(
        Direction::Vertical,
        [Constraint::Length(1), Constraint::default()],
    )
    .split(area);
    let tabs = Tabs::new::<Line>(vec!["t1".into(), "t2".into(), "t3".into()])
        .block(Block::default().borders(Borders::ALL).title("Good"))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tab_selected);
    frame.render_widget(tabs, chunks[0]);

    match app.tab_selected {
        0 => render_task_manager(frame, chunks[1], app),
        1 => render_settings(frame, chunks[1], app),
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
    // TODO timer duration
    // TODO settings module, `timer` `short break` `long break`
    let b = Block::default()
        .title(app.tab_selected.to_string())
        .borders(Borders::LEFT | Borders::RIGHT)
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black));
    frame.render_widget(b, area);
}
