use std::io::stdout;
use terminal;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ColorGlyph {
    pub glyph: char,
    pub foreground_color: Option<terminal::Color>,
    pub background_color: Option<terminal::Color>,
}

pub const EMPTY_COLOR_GLYPH: ColorGlyph = ColorGlyph {
    glyph: ' ',
    foreground_color: None,
    background_color: None,
};

impl ColorGlyph {
    pub fn print(&self) {
        terminal::set_foreground_color(terminal::Color::Default);
        terminal::set_background_color(terminal::Color::Default);
        if let Some(color) = self.foreground_color {
            terminal::set_foreground_color(color);
        }
        if let Some(color) = self.background_color {
            terminal::set_background_color(color);
        }
        print!("{}", self.glyph);
    }
}

pub fn color_to_char(color: &Option<terminal::Color>) -> char {
    match color {
        Some(terminal::Color::DarkGrey) => 'a',
        Some(terminal::Color::Red) => 'r',
        Some(terminal::Color::Green) => 'g',
        Some(terminal::Color::Yellow) => 'y',
        Some(terminal::Color::Blue) => 'b',
        Some(terminal::Color::Magenta) => 'm',
        Some(terminal::Color::Cyan) => 'c',
        Some(terminal::Color::White) => 'w',

        Some(terminal::Color::Black) => 'A',
        Some(terminal::Color::DarkRed) => 'R',
        Some(terminal::Color::DarkGreen) => 'G',
        Some(terminal::Color::DarkYellow) => 'Y',
        Some(terminal::Color::DarkBlue) => 'B',
        Some(terminal::Color::DarkMagenta) => 'M',
        Some(terminal::Color::DarkCyan) => 'C',
        Some(terminal::Color::Grey) => 'W',

        // could do a little error checking here
        _ => ' ',
    }
}
