extern crate crossterm;
use input;
use terminal;

use mode::EditorMode;

pub enum Command {
    Quit,
    MoveCursor(input::Direction),
    Resize(input::Direction, isize),
    SetChar(char),
    SetColor(terminal::Color),
    AddFrame,
    DeleteFrame,
    CycleFrame(isize),
    CycleMode,
    SaveMode,
    // play animation isize number times
}

pub fn handle_input(mode: &EditorMode) -> Option<Command> {
    if let Some(press) = input::get_press() {
        if exit(&press) {
            return Some(Command::Quit);
        } else if cycle_mode(&press) {
            return Some(Command::CycleMode);
        } else if save_mode(&press) {
            return Some(Command::SaveMode);
        } else if let Some(direction) = move_cursor(&press) {
            return Some(Command::MoveCursor(direction));
        } else if let Some((direction, magnitude)) = resize(&press) {
            return Some(Command::Resize(direction, magnitude));
        } else if add_frame(&press) {
            return Some(Command::AddFrame);
        } else if delete_frame(&press) {
            return Some(Command::DeleteFrame);
        } else if let Some(num) = cycle_frame(&press) {
            return Some(Command::CycleFrame(num));
        }

        if *mode == EditorMode::Glyph {
            if let Some(character) = set_glyph(&press) {
                return Some(Command::SetChar(character));
            }
        } else if *mode == EditorMode::Color {
            if let Some(color) = set_color(&press) {
                return Some(Command::SetColor(color));
            }
        }
    }
    return None;
}

fn exit(press: &input::Press) -> bool {
    if press.key == input::Key::Esc {
        return true;
    }
    return false;
}

fn cycle_mode(press: &input::Press) -> bool {
    if press.key == input::Key::Tab {
        return true;
    }
    return false;
}

fn save_mode(press: &input::Press) -> bool {
    if press.key == input::Key::Enter {
        return true;
    }
    return false;
}

fn move_cursor(press: &input::Press) -> Option<input::Direction> {
    if let input::Key::Direction(d) = press.key {
        if press.modifier.is_none() {
            return Some(d);
        }
    }
    return None;
}

fn resize(press: &input::Press) -> Option<(input::Direction, isize)> {
    if let input::Key::Direction(d) = press.key {
        if press.modifier == Some(input::Modifier::Control) {
            return Some((d, 1));
        }
        if press.modifier == Some(input::Modifier::Shift) {
            return Some((d, -1));
        }
    }
    return None;
}

fn add_frame(press: &input::Press) -> bool {
    if press.key == input::Key::Insert {
        return true;
    }
    return false;
}

fn delete_frame(press: &input::Press) -> bool {
    if press.key == input::Key::Delete {
        return true;
    }
    return false;
}

fn cycle_frame(press: &input::Press) -> Option<isize> {
    if press.key == input::Key::PageUp {
        return Some(1);
    } else if press.key == input::Key::PageDown {
        return Some(-1);
    }
    return None;
}

fn set_glyph(press: &input::Press) -> Option<char> {
    if let input::Key::Glyph(g) = press.key {
        return Some(g);
    }
    return None;
}

fn set_color(press: &input::Press) -> Option<terminal::Color> {
    match *press {
        // lighter colors
        input::Press {
            key: input::Key::Glyph('a'),
            modifier: None,
        } => return Some(terminal::Color::DarkGrey),
        input::Press {
            key: input::Key::Glyph('r'),
            modifier: None,
        } => return Some(terminal::Color::Red),
        input::Press {
            key: input::Key::Glyph('g'),
            modifier: None,
        } => return Some(terminal::Color::Green),
        input::Press {
            key: input::Key::Glyph('y'),
            modifier: None,
        } => return Some(terminal::Color::Yellow),
        input::Press {
            key: input::Key::Glyph('b'),
            modifier: None,
        } => return Some(terminal::Color::Blue),
        input::Press {
            key: input::Key::Glyph('m'),
            modifier: None,
        } => return Some(terminal::Color::Magenta),
        input::Press {
            key: input::Key::Glyph('c'),
            modifier: None,
        } => return Some(terminal::Color::Cyan),
        input::Press {
            key: input::Key::Glyph('w'),
            modifier: None,
        } => return Some(terminal::Color::White),
        // darker colors
        input::Press {
            key: input::Key::Glyph('A'),
            modifier: Some(input::Modifier::Shift),
        } => return Some(terminal::Color::Black),
        input::Press {
            key: input::Key::Glyph('R'),
            modifier: Some(input::Modifier::Shift),
        } => return Some(terminal::Color::DarkRed),
        input::Press {
            key: input::Key::Glyph('G'),
            modifier: Some(input::Modifier::Shift),
        } => return Some(terminal::Color::DarkGreen),
        input::Press {
            key: input::Key::Glyph('Y'),
            modifier: Some(input::Modifier::Shift),
        } => return Some(terminal::Color::DarkYellow),
        input::Press {
            key: input::Key::Glyph('B'),
            modifier: Some(input::Modifier::Shift),
        } => return Some(terminal::Color::DarkBlue),
        input::Press {
            key: input::Key::Glyph('M'),
            modifier: Some(input::Modifier::Shift),
        } => return Some(terminal::Color::DarkMagenta),
        input::Press {
            key: input::Key::Glyph('C'),
            modifier: Some(input::Modifier::Shift),
        } => return Some(terminal::Color::DarkCyan),
        input::Press {
            key: input::Key::Glyph('W'),
            modifier: Some(input::Modifier::Shift),
        } => return Some(terminal::Color::Grey),
        _ => return None,
    };
}
