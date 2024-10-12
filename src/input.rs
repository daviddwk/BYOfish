use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    style::Color,
    terminal::{disable_raw_mode, enable_raw_mode, Clear},
    ExecutableCommand,
};

use commands::{Command, Direction};

pub fn handle_blocking_input() -> Command {
    let mut command = Command {
        quit: false,
        move_cursor: None,
        resize: None,
        cycle_frame: None,
        set_char: None,
        set_color: None,
        add_frame: false,
        delete_frame: false,
    };

    // this blocks until somthing happens
    let event = crossterm::event::read().unwrap();

    command.set_char = match_set_char(&event);
    command.set_color = match_set_color(&event);

    match event {
        //
        // EXIT
        //
        Event::Key(KeyEvent {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.quit = true,
        // move
        Event::Key(KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.move_cursor = Some(Direction::Left),
        Event::Key(KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.move_cursor = Some(Direction::Right),
        Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.move_cursor = Some(Direction::Up),
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.move_cursor = Some(Direction::Down),
        //
        // RESIZE
        // grow
        // TODO make a grow / shrink mode for this and for adding / removing frames
        Event::Key(KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.resize = Some((Direction::Left, 1)),
        Event::Key(KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.resize = Some((Direction::Right, 1)),
        Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.resize = Some((Direction::Up, 1)),
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.resize = Some((Direction::Down, 1)),
        // shrink
        Event::Key(KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.resize = Some((Direction::Left, -1)),
        Event::Key(KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.resize = Some((Direction::Right, -1)),
        Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.resize = Some((Direction::Up, -1)),
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.resize = Some((Direction::Down, -1)),
        //
        // FRAMES
        //
        Event::Key(KeyEvent {
            code: KeyCode::Insert,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.add_frame = true,
        Event::Key(KeyEvent {
            code: KeyCode::Delete,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.delete_frame = true,
        Event::Key(KeyEvent {
            code: KeyCode::PageUp,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.cycle_frame = Some(1),
        Event::Key(KeyEvent {
            code: KeyCode::PageDown,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.cycle_frame = Some(-1),
        _ => {}
    };
    return command;
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
