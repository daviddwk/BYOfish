use animation::{blank_animation, load_animation, Animation, Position, Size};
use color_glyph::EMPTY_COLOR_GLYPH;
use color_glyph::{color_to_char, ColorGlyph};
use command;
use input::Direction;
use open_json::open_json;
use serde_json::json;
use std::io::stdout;
use std::path::PathBuf;
use terminal;

pub struct Asset {
    // this probably shouldn't be public
    animation: Animation,
    cursor_position: Position,
    current_frame: usize,
}

impl Asset {
    pub fn new(path: &PathBuf, name: &str) -> Asset {
        // make so you give a path and it opens the file
        //   then it lists the animations and lets you cycle through them with like
        //   page up or down or somthing
        //
        // hardcode to look for forward/flipped_animation as well as
        //   foreground / background animation
        let anim_json = open_json(path, name, "fish");
        let anim: Animation = load_animation(&anim_json, "test fish", "/forward_animation");
        return Asset {
            animation: anim,
            cursor_position: Position { x: 0, y: 0 },
            current_frame: 0,
        };
    }

    pub fn get_size(&self) -> Size {
        return Size {
            width: self.animation[0][0].len(),
            height: self.animation[0].len(),
        };
    }

    pub fn get_frame_idx(&self) -> usize {
        return self.current_frame;
    }

    pub fn get_frame_num(&self) -> usize {
        return self.animation.len();
    }

    pub fn get_cursor_position(&self) -> Position {
        return self.cursor_position;
    }

    pub fn print(&self) {
        let frame_idx = self.current_frame;
        for line_idx in 0..self.get_size().height {
            // print top line
            if line_idx == 0 {
                terminal::set_foreground_color(terminal::Color::Default);
                terminal::set_background_color(terminal::Color::Default);
                print!("┏{}┓ \r\n", "━".repeat(self.get_size().width));
            }
            for glyph_idx in 0..self.get_size().width {
                if glyph_idx == 0 {
                    terminal::set_foreground_color(terminal::Color::Default);
                    terminal::set_background_color(terminal::Color::Default);
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
                if glyph_idx == self.get_size().width - 1 {
                    terminal::set_foreground_color(terminal::Color::Default);
                    terminal::set_background_color(terminal::Color::Default);
                    print!("┃ ");
                }
            }
            print!("\r\n");
            // print bottom line
            if line_idx == self.get_size().height - 1 {
                print!("┗{}┛ \r\n", "━".repeat(self.get_size().width));
            }
        }
    }

    pub fn handle_command(&mut self, cmd: &command::Command) {
        if let Some(mv) = cmd.move_cursor {
            self.move_cursor(&mv);
        } else if let Some(rz) = cmd.resize {
            self.resize(&rz.0, rz.1);
        } else if let Some(glyph) = cmd.set_char {
            self.set_char(glyph);
        } else if let Some(color) = cmd.set_color {
            self.set_color(&color);
        } else if cmd.add_frame {
            self.add_frame();
        } else if cmd.delete_frame {
            self.delete_frame();
        } else if let Some(delta) = cmd.cycle_frame {
            self.cycle_frame(delta);
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
                if self.cursor_position.x < self.get_size().width - 1 {
                    self.cursor_position.x += 1;
                }
            }
            Direction::Up => {
                if self.cursor_position.y >= 1 {
                    self.cursor_position.y -= 1;
                }
            }
            Direction::Down => {
                if self.cursor_position.y < self.get_size().height - 1 {
                    self.cursor_position.y += 1;
                }
            }
        }
    }

    pub fn resize(&mut self, direction: &Direction, delta: isize) {
        let delta_abs = delta.abs();
        let grow = delta.is_positive();

        for _i in 0..delta_abs {
            // change the size
            // size should just be a function shouldn't it
            if *direction == Direction::Up || *direction == Direction::Down {
                if grow {
                    self.get_size().height += 1;
                } else {
                    // so it doesn't shrink to nothing
                    if self.get_size().height <= 1 {
                        break;
                    }
                    self.get_size().height -= 1;
                }
            } else {
                // else must be left or right
                if grow {
                    self.get_size().width += 1;
                } else {
                    // so it doesn't shrink to nothing
                    if self.get_size().width <= 1 {
                        break;
                    }
                    self.get_size().width -= 1;
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
                            if line_idx == 0 && frame_idx == 0 {
                                self.cursor_position.x += 1;
                            }
                        } else {
                            self.animation[frame_idx][line_idx].remove(0);
                            // this is a sign this code sucks
                            if line_idx == 0 && frame_idx == 0 {
                                if self.cursor_position.x != 0 {
                                    self.cursor_position.x -= 1;
                                }
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
        let asset_size: Size = self.get_size();
        if self.cursor_position.x >= asset_size.width {
            self.cursor_position.x = asset_size.width - 1;
        }
        if self.cursor_position.y >= asset_size.height {
            self.cursor_position.y = asset_size.height - 1;
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

    pub fn set_color(&mut self, color: &terminal::Color) {
        let frame_idx = self.current_frame;
        let line_idx = self.cursor_position.y;
        let glyph_idx = self.cursor_position.x;
        let mut color_glyph = self.animation[frame_idx][line_idx][glyph_idx];
        color_glyph.foreground_color = Some(*color);
        self.animation[frame_idx][line_idx][glyph_idx] = color_glyph;
    }

    pub fn cycle_frame(&mut self, delta: isize) {
        let new_frame_idx =
            (self.current_frame as isize + delta).rem_euclid(self.get_frame_num() as isize);
        self.current_frame = new_frame_idx as usize;
    }

    pub fn add_frame(&mut self) {
        self.animation.insert(
            self.current_frame,
            blank_animation(self.get_size())[0].clone(),
        );
    }

    pub fn delete_frame(&mut self) {
        if !(self.animation.len() <= 1) {
            self.animation.remove(self.current_frame);
            self.current_frame = self.current_frame % self.get_size().height;
        }
    }

    pub fn export(&mut self) -> serde_json::Value {
        let size = self.get_size();
        let num_frames = self.get_frame_num();

        let mut forward_animation_symbols: Vec<Vec<String>> = Vec::new();
        let mut forward_animation_colors: Vec<Vec<String>> = Vec::new();
        let mut forward_animation_highlights: Vec<Vec<String>> = Vec::new();

        let mut flipped_animation_symbols: Vec<Vec<String>> = Vec::new();
        let mut flipped_animation_colors: Vec<Vec<String>> = Vec::new();
        let mut flipped_animation_highlights: Vec<Vec<String>> = Vec::new();

        // I should learn that fancy functional stuff
        for frame_idx in 0..num_frames {
            forward_animation_symbols.push(Vec::new());
            forward_animation_colors.push(Vec::new());
            forward_animation_highlights.push(Vec::new());

            flipped_animation_symbols.push(Vec::new());
            flipped_animation_colors.push(Vec::new());
            flipped_animation_highlights.push(Vec::new());

            for line_idx in 0..size.height {
                forward_animation_symbols[frame_idx].push(String::new());
                forward_animation_colors[frame_idx].push(String::new());
                forward_animation_highlights[frame_idx].push(String::new());

                flipped_animation_symbols[frame_idx].push(String::new());
                flipped_animation_colors[frame_idx].push(String::new());
                flipped_animation_highlights[frame_idx].push(String::new());

                for glyph_idx in 0..size.width {
                    let color_glyph = self.animation[frame_idx][line_idx][glyph_idx];
                    forward_animation_symbols[frame_idx][line_idx].push(color_glyph.glyph);
                    forward_animation_colors[frame_idx][line_idx]
                        .push(color_to_char(&color_glyph.foreground_color));
                    forward_animation_highlights[frame_idx][line_idx]
                        .push(color_to_char(&color_glyph.background_color));

                    flipped_animation_symbols[frame_idx][line_idx].push(color_glyph.glyph);
                    flipped_animation_colors[frame_idx][line_idx]
                        .push(color_to_char(&color_glyph.foreground_color));
                    flipped_animation_highlights[frame_idx][line_idx]
                        .push(color_to_char(&color_glyph.background_color));
                }
            }
        }

        return json!({
            "forward_animation": {
                "symbols": forward_animation_symbols,
                "colors": forward_animation_colors,
                "highlights": forward_animation_highlights,
            },
            "flipped_animation": {
                "symbols": flipped_animation_symbols,
                "colors": flipped_animation_colors,
                "highlights": flipped_animation_highlights,
            },
        });
    }
}
