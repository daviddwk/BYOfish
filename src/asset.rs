use animation::{Animation, Position, Size};

pub struct Asset {
    // this probably shouldn't be public
    pub animation: Animation,
    pub size: Size,
    pub cursor_position: Position,
    pub current_frame: usize,
}
