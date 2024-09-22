use std::io::stdout;
use std::process::exit;
extern crate structopt;
use structopt::StructOpt;

extern crate crossterm;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    style::Color,
    terminal::{disable_raw_mode, enable_raw_mode, Clear},
    ExecutableCommand,
};
extern crate colored;
extern crate home;
extern crate rand;
extern crate serde_json;

mod animation;
use animation::{blank_animation, Animation, Position, Size};
mod color_glyph;
use color_glyph::{ColorGlyph, EMPTY_COLOR_GLYPH};
mod error;
use error::error;
mod open_json;

#[derive(StructOpt)]
#[structopt(
    name = "byofish",
    version = "0.0.1",
    about = "Create assets for freefish!"
)]
struct Opt {}

struct Asset {
    animation: Animation,
    size: Size,
    cursor_position: Position,
    current_frame: usize,
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Command {
    quit: bool,
    move_cursor: Option<Direction>,
    resize: Option<(Direction, i32)>,
    cycle_frame: Option<isize>,
    set_char: Option<char>,
    set_color: Option<Color>,
    add_frame: bool,
    delete_frame: bool,
    // play animation isize number times
}

fn main() {
    let args = Opt::from_args();
    let mut asset: Asset = Asset {
        animation: blank_animation(Size {
            height: 3,
            width: 3,
        }),
        size: Size {
            height: 3,
            width: 3,
        },
        cursor_position: Position { x: 0, y: 0 },
        current_frame: 0,
    };

    // init terminal
    enable_raw_mode().unwrap();
    stdout().execute(Hide).unwrap();
    stdout()
        .execute(crossterm::terminal::DisableLineWrap)
        .unwrap();
    stdout()
        .execute(Clear(crossterm::terminal::ClearType::All))
        .unwrap();

    // main loop
    let mut cmd = Command {
        quit: false,
        move_cursor: None,
        resize: None,
        cycle_frame: None,
        set_char: None,
        set_color: None,
        add_frame: false,
        delete_frame: false,
    };
    loop {
        print_asset(&mut asset);
        println!(
            "frame: {}, height:{} width:{}",
            asset.current_frame, asset.size.height, asset.size.width
        );
        println!(
            "x:{} y:{}",
            asset.cursor_position.x, asset.cursor_position.y
        );
        cmd = handle_blocking_input();
        if let Some(mv) = cmd.move_cursor {
            move_cursor(&mut asset, &mv);
        }
        if cmd.quit {
            break;
        }
        if let Some(rz) = cmd.resize {
            resize(&mut asset, &rz.0, rz.1);
        }
        if let Some(glyph) = cmd.set_char {
            set_char(&mut asset, glyph);
        }
        if let Some(color) = cmd.set_color {
            set_color(&mut asset, color);
        }
        if cmd.add_frame {
            add_frame(&mut asset);
        }
        if cmd.delete_frame {
            delete_frame(&mut asset);
        }
        if let Some(delta) = cmd.cycle_frame {
            cycle_frame(&mut asset, delta);
        }
    }
    // return terminal to regular state
    stdout()
        .execute(crossterm::terminal::EnableLineWrap)
        .unwrap();
    stdout().execute(Show).unwrap();
    disable_raw_mode().unwrap();
    exit(0);
}

fn print_asset(asset: &mut Asset) {
    stdout().execute(MoveTo(0, 0)).unwrap();
    let frame_idx = asset.current_frame;
    for line_idx in 0..asset.animation[frame_idx].len() {
        for glyph_idx in 0..asset.animation[frame_idx][line_idx].len() {
            let pos = Position {
                x: glyph_idx,
                y: line_idx,
            };
            if pos == asset.cursor_position {
                ColorGlyph {
                    glyph: 'X',
                    foreground_color: None,
                    background_color: None,
                }
                .print();
            } else {
                asset.animation[frame_idx][line_idx][glyph_idx].print();
            }
        }
        print!("\r\n");
    }
}

fn handle_blocking_input() -> Command {
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

fn move_cursor(asset: &mut Asset, direction: &Direction) {
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

// TODO move cursor appropriately
// TODO don't let it shrink into nothing
fn resize(asset: &mut Asset, direction: &Direction, delta: i32) {
    let delta_abs = delta.abs();
    let grow = delta.is_positive();

    for _i in 0..delta_abs {
        // change the size
        // size should just be a function shouldn't it
        if *direction == Direction::Up || *direction == Direction::Down {
            if grow {
                asset.size.height += 1;
            } else {
                asset.size.height -= 1;
            }
        } else {
            // else must be left or right
            if grow {
                asset.size.width += 1;
            } else {
                asset.size.width -= 1;
            }
        }
        // this could be inverted and it might be better
        for frame_idx in 0..asset.animation.len() {
            if *direction == Direction::Up {
                if grow {
                    let line_len = asset.animation[frame_idx][0].len();
                    asset.animation[frame_idx].insert(0, vec![EMPTY_COLOR_GLYPH; line_len]);
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
}

fn set_char(asset: &mut Asset, glyph: char) {
    let frame_idx = asset.current_frame;
    let line_idx = asset.cursor_position.y;
    let glyph_idx = asset.cursor_position.x;
    let mut color_glyph = asset.animation[frame_idx][line_idx][glyph_idx];
    color_glyph.glyph = glyph;
    asset.animation[frame_idx][line_idx][glyph_idx] = color_glyph;
}

fn set_color(asset: &mut Asset, color: Color) {
    let frame_idx = asset.current_frame;
    let line_idx = asset.cursor_position.y;
    let glyph_idx = asset.cursor_position.x;
    let mut color_glyph = asset.animation[frame_idx][line_idx][glyph_idx];
    color_glyph.foreground_color = Some(color);
    asset.animation[frame_idx][line_idx][glyph_idx] = color_glyph;
}

fn cycle_frame(asset: &mut Asset, delta: isize) {
    let new_frame_idx =
        (asset.current_frame as isize + delta).rem_euclid(asset.animation.len() as isize);
    println!("new_frame_idx: {}", new_frame_idx);
    asset.current_frame = new_frame_idx as usize;
}

fn add_frame(asset: &mut Asset) {
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

fn delete_frame(asset: &mut Asset) {
    if !(asset.animation.len() <= 1) {
        asset.animation.remove(asset.current_frame);
        asset.current_frame = asset.current_frame % asset.animation[0].len();
    }
}
