// TODO Wrap and trim in ratatui
use std::{
    io::{self, stdout},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

pub mod digits_clock;
use crate::digits_clock::*;
pub mod app;
use crate::app::*;

const FPS: u64 = 30;

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
        // app.update_timer();
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
            app.push_user_input_field(code);
            Ok(false)
        }
        (KeyEventKind::Press, KeyCode::Esc, _) => {
            app.clear_input_field();
            Ok(false)
        }
        (KeyEventKind::Press, KeyCode::Backspace, _) => {
            app.pop_user_input_field();
            Ok(false)
        }
        (KeyEventKind::Press, KeyCode::Enter, _) => {
            app.set_timer();
            Ok(false)
        }
        (KeyEventKind::Press, KeyCode::Tab, _) => {
            app.select_next_field();
            Ok(false)
        }
        (KeyEventKind::Press, KeyCode::BackTab, _) => {
            app.select_prev_field();
            Ok(false)
        }
        (KeyEventKind::Press, KeyCode::Char(code), _) => {
            if code == 'q' {
                Ok(true)
            } else if code == 'l' || code == 'h' {
                app.tab_toggle();
                Ok(false)
            } else if code == 'j' {
                app.select_next_field();
                Ok(false)
            } else if code == 'k' {
                app.select_prev_field();
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
    let ((s1, timer), (s2, short_break), (s3, long_break)) = app.display_user_input();
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
