use animation::{blank_animation, load_animation, Animation, Position, Size};
use color_glyph::EMPTY_COLOR_GLYPH;
use color_glyph::{color_to_char, ColorGlyph};
use command;
use input::Direction;
use open_json::open_json;
use serde_json::json;
use std::path::PathBuf;
use terminal;

#[derive(PartialEq, Copy, Clone)]
pub enum Flip {
    FORWARD,
    FLIPPED,
}

pub struct Asset {
    // this probably shouldn't be public
    // TODO manage that these can have different number of frames
    forward_animation: Animation,
    flipped_animation: Animation,
    flip: Flip,
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
        let forward_animation: Animation =
            load_animation(&anim_json, "test fish", "/forward_animation");
        let flipped_animation: Animation =
            load_animation(&anim_json, "test fish", "/flipped_animation");
        return Asset {
            forward_animation,
            flipped_animation,
            flip: Flip::FORWARD,
            cursor_position: Position { x: 0, y: 0 },
            current_frame: 0,
        };
    }

    pub fn get_size(&self) -> Size {
        return Size {
            width: self.forward_animation[0][0].len(),
            height: self.forward_animation[0].len(),
        };
    }

    pub fn get_frame_idx(&self) -> usize {
        return self.current_frame;
    }

    pub fn get_frame_num(&self, flip: Option<Flip>) -> usize {
        match flip {
            Some(Flip::FORWARD) => return self.forward_animation.len(),
            Some(Flip::FLIPPED) => return self.flipped_animation.len(),
            None => return self.current_animation().len(),
        }
    }

    pub fn get_cursor_position(&self) -> Position {
        return self.cursor_position;
    }

    fn current_animation_mut(&mut self) -> &mut Animation {
        match self.flip {
            Flip::FORWARD => return &mut self.forward_animation,
            Flip::FLIPPED => return &mut self.flipped_animation,
        }
    }

    fn current_animation(&self) -> &Animation {
        match self.flip {
            Flip::FORWARD => return &self.forward_animation,
            Flip::FLIPPED => return &self.flipped_animation,
        }
    }

    pub fn print(&self, show_cursor: bool) {
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
                if pos == self.cursor_position && show_cursor {
                    ColorGlyph {
                        glyph: '•',
                        foreground_color: None,
                        background_color: None,
                    }
                    .print();
                } else {
                    self.current_animation()[frame_idx][line_idx][glyph_idx].print();
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

    pub fn print_cursor(&self, show_cursor: bool) {
        terminal::move_cursor(self.cursor_position);
        if show_cursor {
            ColorGlyph {
                glyph: 'X',
                foreground_color: None,
                background_color: None,
            }
            .print();
        } else {
            self.current_animation()[self.current_frame][self.cursor_position.y]
                [self.cursor_position.x]
                .print();
        }
    }

    pub fn handle_command(&mut self, cmd: &command::Command) {
        match cmd {
            command::Command::MoveCursor(direction) => {
                self.move_cursor(direction);
            }
            command::Command::Resize(direction, magnitude) => {
                self.resize(direction, *magnitude);
            }
            command::Command::SetChar(character) => {
                self.set_char(*character);
            }
            command::Command::SetColor(color) => {
                self.set_color(&color);
            }
            command::Command::AddFrame => {
                self.add_frame();
            }
            command::Command::DeleteFrame => {
                self.delete_frame();
            }
            command::Command::CycleFrame(magnitude) => {
                self.cycle_frame(*magnitude);
            }
            _ => {}
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
            for frame_idx in 0..self.current_animation().len() {
                if *direction == Direction::Up {
                    if grow {
                        let line_len = self.current_animation()[frame_idx][0].len();
                        self.forward_animation[frame_idx]
                            .insert(0, vec![EMPTY_COLOR_GLYPH; line_len]);
                        self.flipped_animation[frame_idx]
                            .insert(0, vec![EMPTY_COLOR_GLYPH; line_len]);
                        // cursor moves naturally with growth
                        if frame_idx == 0 {
                            self.cursor_position.y += 1;
                        }
                    } else {
                        // TODO wrap animations together with a grow and shrink function
                        self.current_animation_mut()[frame_idx].remove(0);
                        self.current_animation_mut()[frame_idx].remove(0);
                    }
                } else if *direction == Direction::Down {
                    if grow {
                        let line_len = self.current_animation()[frame_idx][0].len();
                        self.current_animation_mut()[frame_idx]
                            .push(vec![EMPTY_COLOR_GLYPH; line_len]);
                    } else {
                        self.current_animation_mut()[frame_idx].pop();
                    }
                }
                for line_idx in 0..self.current_animation()[frame_idx].len() {
                    if *direction == Direction::Left {
                        if grow {
                            self.forward_animation[frame_idx][line_idx]
                                .insert(0, EMPTY_COLOR_GLYPH);
                            self.flipped_animation[frame_idx][line_idx]
                                .insert(0, EMPTY_COLOR_GLYPH);
                            // cursor moves naturally with growth
                            if line_idx == 0 && frame_idx == 0 {
                                self.cursor_position.x += 1;
                            }
                        } else {
                            self.current_animation_mut()[frame_idx][line_idx].remove(0);
                            self.current_animation_mut()[frame_idx][line_idx].remove(0);
                            // this is a sign this code sucks
                            if line_idx == 0 && frame_idx == 0 {
                                if self.cursor_position.x != 0 {
                                    self.cursor_position.x -= 1;
                                }
                            }
                        }
                    } else if *direction == Direction::Right {
                        if grow {
                            self.current_animation_mut()[frame_idx][line_idx]
                                .push(EMPTY_COLOR_GLYPH);
                        } else {
                            self.current_animation_mut()[frame_idx][line_idx].pop();
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

    pub fn set_char(&mut self, character: char) {
        let frame_idx = self.current_frame;
        let line_idx = self.cursor_position.y;
        let character_idx = self.cursor_position.x;
        let mut color_glyph = self.current_animation_mut()[frame_idx][line_idx][character_idx];
        color_glyph.glyph = character;
        self.current_animation_mut()[frame_idx][line_idx][character_idx] = color_glyph;
    }

    pub fn set_color(&mut self, color: &terminal::Color) {
        let frame_idx = self.current_frame;
        let line_idx = self.cursor_position.y;
        let glyph_idx = self.cursor_position.x;
        let mut color_glyph = self.current_animation_mut()[frame_idx][line_idx][glyph_idx];
        color_glyph.foreground_color = Some(*color);
        self.current_animation_mut()[frame_idx][line_idx][glyph_idx] = color_glyph;
    }

    pub fn cycle_frame(&mut self, delta: isize) {
        let new_frame_idx =
            (self.current_frame as isize + delta).rem_euclid(self.get_frame_num(None) as isize);
        self.current_frame = new_frame_idx as usize;
    }

    pub fn add_frame(&mut self) {
        self.forward_animation.insert(
            self.current_frame,
            blank_animation(self.get_size())[0].clone(),
        );
    }

    pub fn delete_frame(&mut self) {
        if !(self.forward_animation.len() <= 1) {
            self.forward_animation.remove(self.current_frame);
            self.current_frame = self.current_frame % self.forward_animation.len();
        }
    }

    pub fn flip(&mut self) {
        match self.flip {
            Flip::FORWARD => self.flip = Flip::FLIPPED,
            Flip::FLIPPED => self.flip = Flip::FORWARD,
        }
        let frame_num = self.get_frame_num(None);
        if self.current_frame >= frame_num {
            self.current_frame = frame_num - 1;
        }
    }

    pub fn export(&self) -> serde_json::Value {
        // TODO support flipped animation here
        let size = self.get_size();

        let mut forward_animation_symbols: Vec<Vec<String>> = Vec::new();
        let mut forward_animation_colors: Vec<Vec<String>> = Vec::new();
        let mut forward_animation_highlights: Vec<Vec<String>> = Vec::new();

        let mut flipped_animation_symbols: Vec<Vec<String>> = Vec::new();
        let mut flipped_animation_colors: Vec<Vec<String>> = Vec::new();
        let mut flipped_animation_highlights: Vec<Vec<String>> = Vec::new();

        // I should learn that fancy functional stuff
        for frame_idx in 0..self.get_frame_num(Some(Flip::FORWARD)) {
            forward_animation_symbols.push(Vec::new());
            forward_animation_colors.push(Vec::new());
            forward_animation_highlights.push(Vec::new());
            for line_idx in 0..size.height {
                forward_animation_symbols[frame_idx].push(String::new());
                forward_animation_colors[frame_idx].push(String::new());
                forward_animation_highlights[frame_idx].push(String::new());
                for glyph_idx in 0..size.width {
                    let color_glyph = self.forward_animation[frame_idx][line_idx][glyph_idx];
                    forward_animation_symbols[frame_idx][line_idx].push(color_glyph.glyph);
                    forward_animation_colors[frame_idx][line_idx]
                        .push(color_to_char(&color_glyph.foreground_color));
                    forward_animation_highlights[frame_idx][line_idx]
                        .push(color_to_char(&color_glyph.background_color));
                }
            }
        }

        for frame_idx in 0..self.get_frame_num(Some(Flip::FLIPPED)) {
            flipped_animation_symbols.push(Vec::new());
            flipped_animation_colors.push(Vec::new());
            flipped_animation_highlights.push(Vec::new());
            for line_idx in 0..size.height {
                flipped_animation_symbols[frame_idx].push(String::new());
                flipped_animation_colors[frame_idx].push(String::new());
                flipped_animation_highlights[frame_idx].push(String::new());
                for glyph_idx in 0..size.width {
                    let color_glyph = self.flipped_animation[frame_idx][line_idx][glyph_idx];
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
