use animation::{blank_animation, Size};
use asset::Asset;
use color_glyph::EMPTY_COLOR_GLYPH;
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

pub fn move_cursor(asset: &mut Asset, direction: &Direction) {
    match direction {
        Direction::Left => {
            if asset.cursor_position.x >= 1 {
                asset.cursor_position.x -= 1;
            }
        }
        Direction::Right => {
            if asset.cursor_position.x < asset.size.width - 1 {
                asset.cursor_position.x += 1;
            }
        }
        Direction::Up => {
            if asset.cursor_position.y >= 1 {
                asset.cursor_position.y -= 1;
            }
        }
        Direction::Down => {
            if asset.cursor_position.y < asset.size.height - 1 {
                asset.cursor_position.y += 1;
            }
        }
    }
}

pub fn resize(asset: &mut Asset, direction: &Direction, delta: i32) {
    let delta_abs = delta.abs();
    let grow = delta.is_positive();

    for _i in 0..delta_abs {
        // change the size
        // size should just be a function shouldn't it
        if *direction == Direction::Up || *direction == Direction::Down {
            if grow {
                asset.size.height += 1;
            } else {
                // so it doesn't shrink to nothing
                if asset.size.height <= 1 {
                    break;
                }
                asset.size.height -= 1;
            }
        } else {
            // else must be left or right
            if grow {
                asset.size.width += 1;
            } else {
                // so it doesn't shrink to nothing
                if asset.size.width <= 1 {
                    break;
                }
                asset.size.width -= 1;
            }
        }
        // this could be inverted and it might be better
        for frame_idx in 0..asset.animation.len() {
            if *direction == Direction::Up {
                if grow {
                    let line_len = asset.animation[frame_idx][0].len();
                    asset.animation[frame_idx].insert(0, vec![EMPTY_COLOR_GLYPH; line_len]);
                    // cursor moves naturally with growth
                    if frame_idx == 0 {
                        asset.cursor_position.y += 1;
                    }
                } else {
                    asset.animation[frame_idx].remove(0);
                }
            } else if *direction == Direction::Down {
                if grow {
                    let line_len = asset.animation[frame_idx][0].len();
                    asset.animation[frame_idx].push(vec![EMPTY_COLOR_GLYPH; line_len]);
                } else {
                    asset.animation[frame_idx].pop();
                }
            }
            for line_idx in 0..asset.animation[frame_idx].len() {
                if *direction == Direction::Left {
                    if grow {
                        asset.animation[frame_idx][line_idx].insert(0, EMPTY_COLOR_GLYPH);
                        // cursor moves naturally with growth
                        if line_idx == 0 {
                            asset.cursor_position.x += 1;
                        }
                    } else {
                        asset.animation[frame_idx][line_idx].remove(0);
                    }
                } else if *direction == Direction::Right {
                    if grow {
                        asset.animation[frame_idx][line_idx].push(EMPTY_COLOR_GLYPH);
                    } else {
                        asset.animation[frame_idx][line_idx].pop();
                    }
                }
            }
        }
    }
    if asset.cursor_position.x >= asset.animation[0][0].len() {
        asset.cursor_position.x = asset.animation[0][0].len() - 1;
    }
    if asset.cursor_position.y >= asset.animation[0].len() {
        asset.cursor_position.y = asset.animation[0].len() - 1;
    }
}

pub fn set_char(asset: &mut Asset, glyph: char) {
    let frame_idx = asset.current_frame;
    let line_idx = asset.cursor_position.y;
    let glyph_idx = asset.cursor_position.x;
    let mut color_glyph = asset.animation[frame_idx][line_idx][glyph_idx];
    color_glyph.glyph = glyph;
    asset.animation[frame_idx][line_idx][glyph_idx] = color_glyph;
}

pub fn set_color(asset: &mut Asset, color: Color) {
    let frame_idx = asset.current_frame;
    let line_idx = asset.cursor_position.y;
    let glyph_idx = asset.cursor_position.x;
    let mut color_glyph = asset.animation[frame_idx][line_idx][glyph_idx];
    color_glyph.foreground_color = Some(color);
    asset.animation[frame_idx][line_idx][glyph_idx] = color_glyph;
}

pub fn cycle_frame(asset: &mut Asset, delta: isize) {
    let new_frame_idx =
        (asset.current_frame as isize + delta).rem_euclid(asset.animation.len() as isize);
    println!("new_frame_idx: {}", new_frame_idx);
    asset.current_frame = new_frame_idx as usize;
}

pub fn add_frame(asset: &mut Asset) {
    asset.animation.insert(
        asset.current_frame,
        // TODO make get height and width functions
        // because this is pretty dumb
        blank_animation(Size {
            height: asset.animation[0].len(),
            width: asset.animation[0][0].len(),
        })[0]
            .clone(),
    );
}

pub fn delete_frame(asset: &mut Asset) {
    if !(asset.animation.len() <= 1) {
        asset.animation.remove(asset.current_frame);
        asset.current_frame = asset.current_frame % asset.animation[0].len();
    }
}
