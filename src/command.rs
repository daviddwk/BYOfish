extern crate crossterm;
use input;
use terminal;

use mode::EditorMode;

pub struct Command {
    pub quit: bool,
    pub move_cursor: Option<input::Direction>,
    pub resize: Option<(input::Direction, isize)>,
    pub cycle_frame: Option<isize>,
    pub set_char: Option<char>,
    pub set_color: Option<terminal::Color>,
    pub add_frame: bool,
    pub delete_frame: bool,
    pub cycle_mode: bool,
    // play animation isize number times
}

pub fn handle_blocking_input(mode: &EditorMode) -> Command {
    let mut command = Command {
        quit: false,
        move_cursor: None,
        resize: None,
        cycle_frame: None,
        set_char: None,
        set_color: None,
        add_frame: false,
        delete_frame: false,
        cycle_mode: false,
    };

    let press = input::blocking_get_press();

    command.quit = exit(&press);
    command.cycle_mode = cycle_mode(&press);
    command.move_cursor = move_cursor(&press);
    command.resize = resize(&press);
    command.add_frame = add_frame(&press);
    command.delete_frame = delete_frame(&press);
    command.cycle_frame = cycle_frame(&press);

    if *mode == EditorMode::Glyph {
        command.set_char = set_glyph(&press);
    } else if *mode == EditorMode::Color {
        command.set_color = set_color(&press);
    }

    return command;
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

fn move_cursor(press: &input::Press) -> Option<input::Direction> {
    if let input::Key::Direction(d) = press.key {
        return Some(d);
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
