use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    style::Color,
};

use mode::EditorMode;

#[derive(PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct Command {
    pub quit: bool,
    pub move_cursor: Option<Direction>,
    pub resize: Option<(Direction, isize)>,
    pub cycle_frame: Option<isize>,
    pub set_char: Option<char>,
    pub set_color: Option<Color>,
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

    // this blocks until somthing happens
    let event = crossterm::event::read().unwrap();

    command.quit = match_exit(&event);
    command.cycle_mode = match_cycle_mode(&event);
    command.move_cursor = match_move_cursor(&event);
    command.resize = match_resize(&event);
    command.add_frame = match_add_frame(&event);
    command.delete_frame = match_delete_frame(&event);
    command.cycle_frame = match_cycle_frame(&event);

    if *mode == EditorMode::Glyph {
        command.set_char = match_set_char(&event);
    } else if *mode == EditorMode::Color {
        command.set_color = match_set_color(&event);
    }

    return command;
}

fn match_exit(event: &crossterm::event::Event) -> bool {
    if let Event::Key(key_event) = event {
        if let KeyCode::Esc = key_event.code {
            return true;
        }
    }
    return false;
}

fn match_cycle_mode(event: &crossterm::event::Event) -> bool {
    if let Event::Key(key_event) = event {
        if let KeyCode::Tab = key_event.code {
            return true;
        }
    }
    return false;
}

fn match_move_cursor(event: &crossterm::event::Event) -> Option<Direction> {
    match event {
        // move
        Event::Key(KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some(Direction::Left),
        Event::Key(KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some(Direction::Right),
        Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some(Direction::Up),
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some(Direction::Down),
        _ => return None,
    }
}

fn match_resize(event: &crossterm::event::Event) -> Option<(Direction, isize)> {
    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some((Direction::Left, 1)),
        Event::Key(KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some((Direction::Right, 1)),
        Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some((Direction::Up, 1)),
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some((Direction::Down, 1)),
        // shrink
        Event::Key(KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some((Direction::Left, -1)),
        Event::Key(KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some((Direction::Right, -1)),
        Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some((Direction::Up, -1)),
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some((Direction::Down, -1)),
        _ => None,
    }
}

fn match_add_frame(event: &crossterm::event::Event) -> bool {
    if let Event::Key(key_event) = event {
        if let KeyCode::Insert = key_event.code {
            return true;
        }
    }
    return false;
}

fn match_delete_frame(event: &crossterm::event::Event) -> bool {
    if let Event::Key(key_event) = event {
        if let KeyCode::Delete = key_event.code {
            return true;
        }
    }
    return false;
}

fn match_cycle_frame(event: &crossterm::event::Event) -> Option<isize> {
    if let Event::Key(key_event) = event {
        if let KeyCode::PageUp = key_event.code {
            return Some(1);
        } else if let KeyCode::PageDown = key_event.code {
            return Some(-1);
        }
    }
    return None;
}

fn match_set_char(event: &crossterm::event::Event) -> Option<char> {
    if let Event::Key(key_event) = event {
        if let KeyCode::Char(c) = key_event.code {
            return Some(c);
        }
    }
    return None;
}

fn match_set_color(event: &crossterm::event::Event) -> Option<Color> {
    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some(Color::DarkGrey),
        Event::Key(KeyEvent {
            code: KeyCode::Char('r'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some(Color::Red),
        Event::Key(KeyEvent {
            code: KeyCode::Char('g'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some(Color::Green),
        Event::Key(KeyEvent {
            code: KeyCode::Char('y'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some(Color::Yellow),
        Event::Key(KeyEvent {
            code: KeyCode::Char('b'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some(Color::Blue),
        Event::Key(KeyEvent {
            code: KeyCode::Char('m'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some(Color::Magenta),
        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some(Color::Cyan),
        Event::Key(KeyEvent {
            code: KeyCode::Char('w'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some(Color::White),
        // dark colors
        Event::Key(KeyEvent {
            code: KeyCode::Char('A'),
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some(Color::Black),
        Event::Key(KeyEvent {
            code: KeyCode::Char('R'),
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some(Color::DarkRed),
        Event::Key(KeyEvent {
            code: KeyCode::Char('G'),
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some(Color::DarkGreen),
        Event::Key(KeyEvent {
            code: KeyCode::Char('Y'),
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some(Color::DarkYellow),
        Event::Key(KeyEvent {
            code: KeyCode::Char('B'),
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some(Color::DarkBlue),
        Event::Key(KeyEvent {
            code: KeyCode::Char('M'),
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some(Color::DarkMagenta),
        Event::Key(KeyEvent {
            code: KeyCode::Char('C'),
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some(Color::DarkCyan),
        Event::Key(KeyEvent {
            code: KeyCode::Char('W'),
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Some(Color::Grey),
        _ => return None,
    };
}
