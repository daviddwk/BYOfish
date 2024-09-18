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
    cycle_frame: Option<i32>,
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
    loop {
        print_asset(&mut asset, 0);
        println!("height:{} width:{}", asset.size.height, asset.size.width);
        println!(
            "x:{} y:{}",
            asset.cursor_position.x, asset.cursor_position.y
        );
        let cmd = handle_blocking_input();
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
            set_char(&mut asset, glyph)
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

fn print_asset(asset: &mut Asset, frame_idx: usize) {
    stdout().execute(MoveTo(0, 0)).unwrap();
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
