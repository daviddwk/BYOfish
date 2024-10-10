use crossterm::style::Color;

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
    pub resize: Option<(Direction, i32)>,
    pub cycle_frame: Option<isize>,
    pub set_char: Option<char>,
    pub set_color: Option<Color>,
    pub add_frame: bool,
    pub delete_frame: bool,
    // play animation isize number times
}
