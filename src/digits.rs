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
