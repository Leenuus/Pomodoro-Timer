use ratatui::{prelude::*, widgets::*};
use crate::App;

pub const ZERO: &str = " 0000
00  00
00  00
00  00
 0000";

pub const ONE: &str = "1111
  11
  11
  11
111111";

pub const TWO: &str = " 2222
22  22
   22
  22
222222";

pub const THREE: &str = " 3333
33  33
   333
33  33
 3333";

pub const FOUR: &str = "44  44
44  44
444444
    44
    44";

pub const FIVE: &str = "555555
55
55555
    55
55555";

pub const SIX: &str = " 6666
66
66666
66  66
 6666";

pub const SEVEN: &str = "777777
   77
  77
 77
77";

pub const EIGHT: &str = " 8888
88  88
 8888
88  88
 8888";

pub const NINE: &str = " 9999
99  99
 99999
    99
 9999";

pub fn time_convert(secs: u64) -> (&'static str, &'static str, &'static str, &'static str) {
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

pub fn get_digit(num: u64) -> &'static str {
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


pub fn render_digit(num: &str, border: Borders) -> Paragraph<'_> {
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

// TODO not only timer, also __stop watch__
// TODO Yet one more feature, different style of clock using `tab` feature
pub fn render_digit_clock(frame: &mut Frame, area: Rect, app: &App) {
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

    // TODO timer is not start
    let (d1, d2, d3, d4) = match app.start_time{
        Some(start_time) => {
            let time_left = app.timer.timer.saturating_sub(start_time.elapsed()).as_secs();
            time_convert(time_left)
        }
        None => (ZERO, ZERO, ZERO, ZERO)
    };

    render_clock_digit(frame, layout[0], d1, 0);

    render_clock_digit(frame, layout[1], d2, 1);

    render_clock_digit(frame, layout[2], d3, 2);

    render_clock_digit(frame, layout[3], d4, 3);
}

pub fn render_clock_digit(frame: &mut Frame, layout: Rect, digit: &str, index: u8) {
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

