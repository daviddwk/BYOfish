use animation::{blank_animation, Animation, Position, Size};
use color_glyph::ColorGlyph;
use color_glyph::EMPTY_COLOR_GLYPH;
use commands::Direction;
use crossterm::style::Color;
use crossterm::{cursor::MoveTo, ExecutableCommand};
use std::io::stdout;

pub struct Asset {
    // this probably shouldn't be public
    pub animation: Animation,
    pub size: Size,
    pub cursor_position: Position,
    pub current_frame: usize,
}

impl Asset {
    pub fn print(&self) {
        stdout().execute(MoveTo(0, 0)).unwrap();
        let frame_idx = self.current_frame;
        for line_idx in 0..self.size.height {
            // print top line
            if line_idx == 0 {
                print!("┏{}┓ \r\n", "━".repeat(self.size.width));
            }
            for glyph_idx in 0..self.size.width {
                if glyph_idx == 0 {
                    print!("┃");
                }
                let pos = Position {
                    x: glyph_idx,
                    y: line_idx,
                };
                // TODO make cursor flashing
                if pos == self.cursor_position {
                    ColorGlyph {
                        glyph: 'X',
                        foreground_color: None,
                        background_color: None,
                    }
                    .print();
                } else {
                    self.animation[frame_idx][line_idx][glyph_idx].print();
                }
                if glyph_idx == self.size.width - 1 {
                    print!("┃ ");
                }
            }
            print!("\r\n");
            // print bottom line
            if line_idx == self.size.height - 1 {
                print!("┗{}┛ \r\n", "━".repeat(self.size.width));
            }
        }
    }
    pub fn move_cursor(&mut self, direction: &Direction) {
        match direction {
            Direction::Left => {
                if self.cursor_position.x >= 1 {
                    self.cursor_position.x -= 1;
                }
            }
            Direction::Right => {
                if self.cursor_position.x < self.size.width - 1 {
                    self.cursor_position.x += 1;
                }
            }
            Direction::Up => {
                if self.cursor_position.y >= 1 {
                    self.cursor_position.y -= 1;
                }
            }
            Direction::Down => {
                if self.cursor_position.y < self.size.height - 1 {
                    self.cursor_position.y += 1;
                }
            }
        }
    }

    pub fn resize(&mut self, direction: &Direction, delta: i32) {
        let delta_abs = delta.abs();
        let grow = delta.is_positive();

        for _i in 0..delta_abs {
            // change the size
            // size should just be a function shouldn't it
            if *direction == Direction::Up || *direction == Direction::Down {
                if grow {
                    self.size.height += 1;
                } else {
                    // so it doesn't shrink to nothing
                    if self.size.height <= 1 {
                        break;
                    }
                    self.size.height -= 1;
                }
            } else {
                // else must be left or right
                if grow {
                    self.size.width += 1;
                } else {
                    // so it doesn't shrink to nothing
                    if self.size.width <= 1 {
                        break;
                    }
                    self.size.width -= 1;
                }
            }
            // this could be inverted and it might be better
            for frame_idx in 0..self.animation.len() {
                if *direction == Direction::Up {
                    if grow {
                        let line_len = self.animation[frame_idx][0].len();
                        self.animation[frame_idx].insert(0, vec![EMPTY_COLOR_GLYPH; line_len]);
                        // cursor moves naturally with growth
                        if frame_idx == 0 {
                            self.cursor_position.y += 1;
                        }
                    } else {
                        self.animation[frame_idx].remove(0);
                    }
                } else if *direction == Direction::Down {
                    if grow {
                        let line_len = self.animation[frame_idx][0].len();
                        self.animation[frame_idx].push(vec![EMPTY_COLOR_GLYPH; line_len]);
                    } else {
                        self.animation[frame_idx].pop();
                    }
                }
                for line_idx in 0..self.animation[frame_idx].len() {
                    if *direction == Direction::Left {
                        if grow {
                            self.animation[frame_idx][line_idx].insert(0, EMPTY_COLOR_GLYPH);
                            // cursor moves naturally with growth
                            if line_idx == 0 {
                                self.cursor_position.x += 1;
                            }
                        } else {
                            self.animation[frame_idx][line_idx].remove(0);
                            // this is a sign this code sucks
                            if line_idx == 0 {
                                self.cursor_position.x -= 1;
                            }
                        }
                    } else if *direction == Direction::Right {
                        if grow {
                            self.animation[frame_idx][line_idx].push(EMPTY_COLOR_GLYPH);
                        } else {
                            self.animation[frame_idx][line_idx].pop();
                        }
                    }
                }
            }
        }
        if self.cursor_position.x >= self.animation[0][0].len() {
            self.cursor_position.x = self.animation[0][0].len() - 1;
        }
        if self.cursor_position.y >= self.animation[0].len() {
            self.cursor_position.y = self.animation[0].len() - 1;
        }
    }

    pub fn set_char(&mut self, glyph: char) {
        let frame_idx = self.current_frame;
        let line_idx = self.cursor_position.y;
        let glyph_idx = self.cursor_position.x;
        let mut color_glyph = self.animation[frame_idx][line_idx][glyph_idx];
        color_glyph.glyph = glyph;
        self.animation[frame_idx][line_idx][glyph_idx] = color_glyph;
    }

    pub fn set_color(&mut self, color: Color) {
        let frame_idx = self.current_frame;
        let line_idx = self.cursor_position.y;
        let glyph_idx = self.cursor_position.x;
        let mut color_glyph = self.animation[frame_idx][line_idx][glyph_idx];
        color_glyph.foreground_color = Some(color);
        self.animation[frame_idx][line_idx][glyph_idx] = color_glyph;
    }

    pub fn cycle_frame(&mut self, delta: isize) {
        let new_frame_idx =
            (self.current_frame as isize + delta).rem_euclid(self.animation.len() as isize);
        println!("new_frame_idx: {}", new_frame_idx);
        self.current_frame = new_frame_idx as usize;
    }

    pub fn add_frame(&mut self) {
        self.animation.insert(
            self.current_frame,
            // TODO make get height and width functions
            // because this is pretty dumb
            blank_animation(Size {
                height: self.animation[0].len(),
                width: self.animation[0][0].len(),
            })[0]
                .clone(),
        );
    }

    pub fn delete_frame(&mut self) {
        if !(self.animation.len() <= 1) {
            self.animation.remove(self.current_frame);
            self.current_frame = self.current_frame % self.animation[0].len();
        }
    }
}
