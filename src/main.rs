use std::{
    io::{self, stdout},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

mod digits_clock;
use crate::digits_clock::*;
mod app;
use crate::app::*;
mod input;
use crate::input::handle_events;

const FPS: u64 = 30;

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    let _program = args.next().unwrap();
    let fps = match args.next() {
        // TODO fix strange Rust error type conversion
        Some(fps) => fps.parse::<u64>().expect("Invalid FPS"),
        None => FPS,
    };
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut should_quit = false;
    let interval: u64 = 1000 / fps;

    let mut app = App::default(); // initialize App

    while !should_quit {
        terminal.draw(|frame| ui(frame, &mut app))?;
        should_quit = handle_events(&mut app)?;
        sleep(Duration::from_millis(interval));
        app.update();
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn ui(frame: &mut Frame, app: &mut App) {
    let layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Ratio(1, 4), Constraint::Ratio(3, 4)],
    )
    .split(frame.size());
    render_task_list(frame, layout[0], app);
    render_right_side(frame, layout[1], app);
}

fn render_task_list(frame: &mut Frame, area: Rect, app: &mut App) {
    let list = List::new(app.task_list.items.iter().map(|t| t.title()))
        .block(
            Block::default()
                .title("Task List")
                .borders(Borders::ALL)
                .title_position(block::Position::Top)
                .title_alignment(Alignment::Center),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::ITALIC)
                .bg(Color::Red)
                .fg(Color::Green),
        )
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(list, area, &mut app.task_list.state);
}

fn render_right_side(frame: &mut Frame, area: Rect, app: &App) {
    let rs = Layout::new(
        Direction::Vertical,
        [Constraint::Ratio(3, 5), Constraint::Ratio(2, 5)],
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

    render_state_prompt(frame, layout[0]);
    render_user_input_fields(frame, layout[1], app);
}

#[allow(unused)]
fn render_help_screen(frame: &mut Frame, area: Rect) {
    // TODO we can generate help from keymap
    todo!()
}

fn render_state_prompt(frame: &mut Frame, area: Rect) {
    // TODO usage prompt widget: render it with dynamic keymaps
    let d1 = Block::default()
        .title("Usage")
        .borders(Borders::ALL)
        .style(Style::default());
    frame.render_widget(d1, area);
}

fn render_user_input_fields(frame: &mut Frame, area: Rect, app: &App) {
    match app.tab_selected {
        app::Tabs::PomodoroSetting => render_settings(frame, area, app),
        app::Tabs::TaskManager => render_task_manager(frame, area, app),
    };
}

// HACK keybindings or button to + or - number input
fn render_task_manager(frame: &mut Frame, area: Rect, app: &App) {
    // TODO task manager input fields
    let text = vec![
        Line::from(vec![Span::styled(
            "Task Name: ",
            Style::new().green().italic(),
        )]),
        Line::default(),
        Line::from(vec![Span::styled(
            "Est Pomodoros: ",
            Style::new().green().italic(),
        )]),
        Line::default(),
        Line::from(vec![Span::styled(
            "Notes(optional): ",
            Style::new().green().italic(),
        )]),
    ];
    let b = Paragraph::new(text)
        .block(
            Block::new()
                .title("Task Manager --> Pomodoro Setting")
                .borders(Borders::ALL)
                .padding(Padding::default()),
        )
        .style(Style::default())
        .alignment(Alignment::Left);
    frame.render_widget(b, area);
}

fn render_settings(frame: &mut Frame, area: Rect, app: &App) {
    let ((s1, timer), (s2, short_break), (s3, long_break), (s4, pomodoro_per_long_break)) =
        app.display_user_input();
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
        Line::default(),
        Line::from(vec![
            Span::styled(s4, Style::new().green().italic()),
            Span::from(pomodoro_per_long_break).style(Style::default()),
        ]),
    ];
    let b = Paragraph::new(text)
        .block(
            Block::new()
                .title("Pomodoro Settings --> Task Manager")
                .borders(Borders::ALL)
                .padding(Padding::default()),
        )
        .style(Style::default())
        .alignment(Alignment::Left);
    // .wrap(Wrap { trim: true });
    frame.render_widget(b, area);
}
