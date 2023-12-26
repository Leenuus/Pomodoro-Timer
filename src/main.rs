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

const FOUR: &str = "44  44
44  44
444444
    44
    44";

const FIVE: &str = "555555
55
55555
    55
55555";

const SIX: &str = " 6666
66
66666
66  66
 6666";

const SEVEN: &str = "777777
   77
  77
 77
77";

const EIGHT: &str = " 8888
88  88
 8888
88  88
 8888";

const NINE: &str = " 9999
99  99
 99999
    99
 9999";

struct App {
    timer: Duration,
    start_time: Instant,
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut should_quit = false;

    // TODO, how can we keep track of the time
    let timer = Duration::from_secs(60 * 25);
    let start_time = Instant::now();

    let app = App { timer, start_time };
    while !should_quit {
        terminal.draw(|frame| ui(frame, &app))?;
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

fn render_digit(num: &str, border: Borders) -> Paragraph<'_> {
    let num: Vec<_> = num.lines().collect();
    // #ba4949
    let text: Vec<_> = num.iter().map(|&line| Line::from(line)).collect();
    // TODO colorscheme design
    // let red = Color::Rgb(0xba, 0x49, 0x49);
    let res = Paragraph::new(text)
        .block(Block::new().borders(border))
        .style(Style::default())
        // TODO fix alignment misprint digits
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    res
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

    render_console(frame, rs[1]);
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

    // TODO get current left time from APP, convert it into corresponding digit string
    let time_left = app.timer.saturating_sub(app.start_time.elapsed()).as_secs();
    let (d1, d2, d3, d4) = time_convert(time_left);
    render_clock_digit(frame, layout[0], d1, 0);

    render_clock_digit(frame, layout[1], d2, 1);

    render_clock_digit(frame, layout[2], d3, 2);

    render_clock_digit(frame, layout[3], d4, 3);
}

fn render_console(frame: &mut Frame, area: Rect) {
    // ID 2
    // TODO console
    let d1 = Block::default()
        .title("Console")
        .borders(Borders::ALL)
        .style(Style::default());
    frame.render_widget(d1, area);
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

fn time_convert(secs: u64) -> (&'static str, &'static str, &'static str, &'static str) {
    let minutes = secs / 60;
    let secs = secs % 60;

    let (m0, m1) = match minutes {
        // FIXME ONLY two digit clock is supported now
        x if x < 100 => (get_digit(x / 10), get_digit(x % 10)),
        _ => panic!(),
    };

    let (s0, s1) = match secs {
        x if x < 60 => (get_digit(x / 10), get_digit(x % 10)),
        _ => panic!(),
    };
    (m0, m1, s0, s1)
}

fn get_digit(num: u64) -> &'static str {
    match num {
        0 => ZERO,
        1 => ONE,
        2 => TWO,
        3 => THREE,
        4 => FOUR,
        5 => FIVE,
        6 => SIX,
        7 => SEVEN,
        8 => EIGHT,
        9 => NINE,
        _ => {
            panic!()
        }
    }
}
