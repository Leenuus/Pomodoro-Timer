use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

const ZERO: &str = " 0000
00  00
00  00
00  00
 0000";

const ONE: &str = "1111
  11
  11
  11
111111";

const TWO: &str = " 2222
22  22
   22
  22
222222";

const THREE: &str = " 3333
33  33
   333
33  33
 3333";

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        match event::read()? {
            Event::Key(key) => {
                return handle_key(key);
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

/// handle basic input
fn handle_key(key: KeyEvent) -> io::Result<bool> {
    if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
        return Ok(true);
    }
    Ok(false)
}

fn digit<'a>(num: &'a str) -> Paragraph<'a> {
    let num: Vec<_> = num.lines().collect();
    // #ba4949
    let text: Vec<_> = num.iter().map(|&line| Line::from(line)).collect();
    // TODO colorscheme design
    // let red = Color::Rgb(0xba, 0x49, 0x49);
    let res = Paragraph::new(text)
        .block(Block::new().title("Pomodoro").borders(Borders::ALL))
        .style(Style::new().bg(Color::Black).fg(Color::White))
        // TODO fix alignment misprint digits
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    res
}

fn ui(frame: &mut Frame) {
    let layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Ratio(1, 4), Constraint::Ratio(3, 4)],
    )
    .split(frame.size());

    // TODO task list
    let d1 = digit(ZERO);
    // render task list
    frame.render_widget(d1, layout[0]);
    // render digit clock
    render_right_side(frame, layout[1]);
}

fn render_right_side(frame: &mut Frame, area: Rect) {
    let rs = Layout::new(
        Direction::Vertical,
        [Constraint::Ratio(3, 4), Constraint::Ratio(1, 4)],
    )
    .split(area);

    render_digit_clock(frame, rs[0]);

    render_console(frame, rs[1]);
}

fn render_digit_clock(frame: &mut Frame, area: Rect) {
    // TODO digit clock
    let d1 = digit(ONE);
    frame.render_widget(d1, area);
}

fn render_console(frame: &mut Frame, area: Rect) {
    // TODO console
    let d1 = digit(TWO);
    frame.render_widget(d1, area);
}
