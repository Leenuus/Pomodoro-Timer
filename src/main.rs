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
        terminal.draw(|frame| ui(frame, &app))?;
        should_quit = handle_events(&mut app)?;
        sleep(Duration::from_millis(interval));
        app.update();
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
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
    let items = ["Unix Programming", "Pomodoro Dev", "Computer Networking"];
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

    render_state_prompt(frame, layout[0]);
    render_user_input_fields(frame, layout[1], app);
}

#[allow(unused)]
fn render_help_screen(frame: &mut Frame, area: Rect){
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
        Line::from(vec![
            Span::styled(s4, Style::new().green().italic()),
            Span::from(pomodoro_per_long_break).style(Style::default()),
        ]),
    ];
    let b = Paragraph::new(text)
        .block(
            Block::new()
                .title("Settings")
                .borders(Borders::ALL)
                .padding(Padding::default()),
        )
        .style(Style::default())
        .alignment(Alignment::Left);
    // .wrap(Wrap { trim: true });
    frame.render_widget(b, area);
}
