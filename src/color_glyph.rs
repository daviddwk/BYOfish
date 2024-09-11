use crossterm::{
    style::{Color, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::stdout;

#[derive(Clone, PartialEq, Debug)]
pub struct ColorGlyph {
    pub glyph: char,
    pub foreground_color: Option<Color>,
    pub background_color: Option<Color>,
}

pub const EMPTY_COLOR_GLYPH: ColorGlyph = ColorGlyph {
    glyph: '.',
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
