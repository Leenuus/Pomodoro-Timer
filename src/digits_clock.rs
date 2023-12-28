use crate::App;
use ratatui::{prelude::*, widgets::*};

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

    // NOTE it is not a good idea to set a timer more than 99 minutes
    // we make sure this function never receive a three-digit number
    // in `crate::App::set_timer`
    let (m0, m1) = match minutes {
        x if x < 100 => (get_digit(x / 10), get_digit(x % 10)),
        _ => unreachable!()
    };

    let (s0, s1) = match secs {
        x if x < 60 => (get_digit(x / 10), get_digit(x % 10)),
        _ => unreachable!()
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
    let text: Vec<_> = num.iter().map(|&line| Line::from(line)).collect();
    let res = Paragraph::new(text)
        // FIXME these digits don't display as expect when the screen size is very small
        // we need dynamic padding aware of the width of terminal
        .block(Block::new().borders(border).padding(Padding::horizontal(8)))
        .style(Style::default())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });
    res
}

// width: 129, height: 33
pub fn render_digit_clock(frame: &mut Frame, area: Rect, app: &App) {
    let layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            // TODO add `:` separator
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
        ],
    )
    .split(area);

    let (d1, d2, d3, d4) = time_convert(app.get_time_left());

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
        1 | 2 => (Borders::TOP, Borders::BOTTOM, Borders::NONE),
        3 => (
            Borders::TOP | Borders::RIGHT,
            Borders::BOTTOM | Borders::RIGHT,
            Borders::RIGHT,
        ),
        _ => {
            unreachable!()
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
