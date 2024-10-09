use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    style::Color,
    terminal::{disable_raw_mode, enable_raw_mode, Clear},
    ExecutableCommand,
};

use commands::{Command, Direction};

pub fn handle_blocking_input() -> Command {
    // blocking read
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

    match read().unwrap() {
        // TODO: Capital Q does not work here
        Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.quit = true,
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
        //
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
        // set char
        // this could definitely be a macro maybe idk
        Event::Key(KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.set_char = Some('a'),
        Event::Key(KeyEvent {
            code: KeyCode::Char('b'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.set_char = Some('b'),
        // set color
        // todo make switch between mode for colors
        Event::Key(KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.set_color = Some(Color::DarkGrey),
        Event::Key(KeyEvent {
            code: KeyCode::Char('A'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.set_color = Some(Color::Black),
        Event::Key(KeyEvent {
            code: KeyCode::Char('r'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.set_color = Some(Color::Red),
        Event::Key(KeyEvent {
            code: KeyCode::Char('R'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => command.set_color = Some(Color::DarkRed),

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

        _ => (),
    };
    return command;
}
