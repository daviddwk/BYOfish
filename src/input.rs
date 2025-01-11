extern crate crossterm;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq)]
pub enum Key {
    Esc,
    Tab,
    Insert,
    Delete,
    Home,
    End,
    Enter,
    Backspace,
    PageUp,
    PageDown,
    Direction(Direction),
    Glyph(char),
}

#[derive(PartialEq)]
pub enum Modifier {
    Control,
    Shift,
    Alt,
    Super,
}

#[derive(PartialEq)]
pub struct Press {
    pub key: Key,
    pub modifier: Option<Modifier>,
}

impl Press {
    pub fn new(key: Key, modifier: Option<Modifier>) -> Press {
        return Press { key, modifier };
    }
}

pub fn get_press() -> Option<Press> {
    loop {
        if !crossterm::event::poll(std::time::Duration::from_millis(10)).unwrap() {
            return None;
        }
        // gauranteed to be Some
        if let crossterm::event::Event::Key(key_event) = crossterm::event::read().unwrap() {
            if let crossterm::event::KeyCode::Char(c) = key_event.code {
                return Some(Press::new(
                    Key::Glyph(c),
                    from_crossterm_modifier(key_event.modifiers),
                ));
            }
            match &key_event {
                crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::Left,
                    ..
                } => {
                    return Some(Press::new(
                        Key::Direction(Direction::Left),
                        from_crossterm_modifier(key_event.modifiers),
                    ));
                }
                crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::Right,
                    ..
                } => {
                    return Some(Press::new(
                        Key::Direction(Direction::Right),
                        from_crossterm_modifier(key_event.modifiers),
                    ));
                }
                crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::Up,
                    ..
                } => {
                    return Some(Press::new(
                        Key::Direction(Direction::Up),
                        from_crossterm_modifier(key_event.modifiers),
                    ));
                }
                crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::Down,
                    ..
                } => {
                    return Some(Press::new(
                        Key::Direction(Direction::Down),
                        from_crossterm_modifier(key_event.modifiers),
                    ));
                }
                crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::Esc,
                    ..
                } => {
                    return Some(Press::new(
                        Key::Esc,
                        from_crossterm_modifier(key_event.modifiers),
                    ));
                }
                crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::Tab,
                    ..
                } => {
                    return Some(Press::new(
                        Key::Tab,
                        from_crossterm_modifier(key_event.modifiers),
                    ));
                }
                crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::Insert,
                    ..
                } => {
                    return Some(Press::new(
                        Key::Insert,
                        from_crossterm_modifier(key_event.modifiers),
                    ));
                }
                crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::Delete,
                    ..
                } => {
                    return Some(Press::new(
                        Key::Delete,
                        from_crossterm_modifier(key_event.modifiers),
                    ));
                }
                crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::Home,
                    ..
                } => {
                    return Some(Press::new(
                        Key::Home,
                        from_crossterm_modifier(key_event.modifiers),
                    ));
                }
                crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::End,
                    ..
                } => {
                    return Some(Press::new(
                        Key::End,
                        from_crossterm_modifier(key_event.modifiers),
                    ));
                }
                crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::PageUp,
                    ..
                } => {
                    return Some(Press::new(
                        Key::PageUp,
                        from_crossterm_modifier(key_event.modifiers),
                    ));
                }
                crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::PageDown,
                    ..
                } => {
                    return Some(Press::new(
                        Key::PageDown,
                        from_crossterm_modifier(key_event.modifiers),
                    ));
                }
                crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::Backspace,
                    ..
                } => {
                    return Some(Press::new(
                        Key::Backspace,
                        from_crossterm_modifier(key_event.modifiers),
                    ));
                }
                crossterm::event::KeyEvent {
                    code: crossterm::event::KeyCode::Enter,
                    ..
                } => {
                    return Some(Press::new(
                        Key::Enter,
                        from_crossterm_modifier(key_event.modifiers),
                    ));
                }
                _ => (),
            }
        }
    }
}

fn from_crossterm_modifier(modifier: crossterm::event::KeyModifiers) -> Option<Modifier> {
    match modifier {
        crossterm::event::KeyModifiers::SHIFT => return Some(Modifier::Shift),
        crossterm::event::KeyModifiers::CONTROL => return Some(Modifier::Control),
        crossterm::event::KeyModifiers::ALT => return Some(Modifier::Alt),
        crossterm::event::KeyModifiers::SUPER => return Some(Modifier::Super),
        _ => return None,
    }
}
