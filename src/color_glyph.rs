use crossterm::{
    style::{Color, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::stdout;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ColorGlyph {
    pub glyph: char,
    pub foreground_color: Option<Color>,
    pub background_color: Option<Color>,
}

pub const EMPTY_COLOR_GLYPH: ColorGlyph = ColorGlyph {
    glyph: ' ',
    foreground_color: None,
    background_color: None,
};

impl ColorGlyph {
    pub fn print(&self) {
        stdout().execute(SetForegroundColor(Color::Reset)).unwrap();
        stdout().execute(SetBackgroundColor(Color::Reset)).unwrap();
        if let Some(fg) = self.foreground_color {
            stdout().execute(SetForegroundColor(fg)).unwrap();
        }
        if let Some(bg) = self.background_color {
            stdout().execute(SetBackgroundColor(bg)).unwrap();
        }
        print!("{}", self.glyph);
    }
}

pub fn color_to_char(color: &Option<Color>) -> char {
    match color {
        Some(Color::DarkGrey) => 'a',
        Some(Color::Red) => 'r',
        Some(Color::Green) => 'g',
        Some(Color::Yellow) => 'y',
        Some(Color::Blue) => 'b',
        Some(Color::Magenta) => 'm',
        Some(Color::Cyan) => 'c',
        Some(Color::White) => 'w',

        Some(Color::Black) => 'A',
        Some(Color::DarkRed) => 'R',
        Some(Color::DarkGreen) => 'G',
        Some(Color::DarkYellow) => 'Y',
        Some(Color::DarkBlue) => 'B',
        Some(Color::DarkMagenta) => 'M',
        Some(Color::DarkCyan) => 'C',
        Some(Color::Grey) => 'W',

        // could do a little error checking here
        _ => ' ',
    }
}
