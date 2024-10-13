extern crate crossterm;
use self::crossterm::ExecutableCommand;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    Black,
    DarkGrey,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    DarkRed,
    DarkGreen,
    DarkYellow,
    DarkBlue,
    DarkMagenta,
    DarkCyan,
    Grey,
    Default,
}

pub fn init() {
    crossterm::terminal::enable_raw_mode().unwrap();
    std::io::stdout().execute(crossterm::cursor::Hide).unwrap();
    std::io::stdout()
        .execute(crossterm::terminal::DisableLineWrap)
        .unwrap();
    std::io::stdout()
        .execute(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::All,
        ))
        .unwrap();
}

pub fn reset() {
    std::io::stdout()
        .execute(crossterm::terminal::EnableLineWrap)
        .unwrap();
    std::io::stdout().execute(crossterm::cursor::Show).unwrap();
    crossterm::terminal::disable_raw_mode().unwrap();
}

pub fn home_cursor() {
    std::io::stdout()
        .execute(crossterm::cursor::MoveTo(0, 0))
        .unwrap();
}

pub fn set_foreground_color(color: Color) {
    std::io::stdout()
        .execute(crossterm::style::SetForegroundColor(to_crossterm_color(
            color,
        )))
        .unwrap();
}

pub fn set_background_color(color: Color) {
    std::io::stdout()
        .execute(crossterm::style::SetBackgroundColor(to_crossterm_color(
            color,
        )))
        .unwrap();
}

fn to_crossterm_color(color: Color) -> crossterm::style::Color {
    match color {
        Color::Black => return crossterm::style::Color::Black,
        Color::DarkGrey => return crossterm::style::Color::DarkGrey,
        Color::Red => return crossterm::style::Color::Red,
        Color::Green => return crossterm::style::Color::Green,
        Color::Yellow => return crossterm::style::Color::Yellow,
        Color::Blue => return crossterm::style::Color::Blue,
        Color::Magenta => return crossterm::style::Color::Magenta,
        Color::Cyan => return crossterm::style::Color::Cyan,
        Color::White => return crossterm::style::Color::White,
        Color::DarkRed => return crossterm::style::Color::DarkRed,
        Color::DarkGreen => return crossterm::style::Color::DarkGreen,
        Color::DarkYellow => return crossterm::style::Color::DarkYellow,
        Color::DarkBlue => return crossterm::style::Color::DarkBlue,
        Color::DarkMagenta => return crossterm::style::Color::DarkMagenta,
        Color::DarkCyan => return crossterm::style::Color::DarkCyan,
        Color::Grey => return crossterm::style::Color::Grey,
        Color::Default => return crossterm::style::Color::Reset,
    }
}
