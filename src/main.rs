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
}

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
            height: 10,
            width: 10,
        }),
        size: Size {
            height: 10,
            width: 10,
        },
        cursor_position: Position { x: 0, y: 0 },
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
        let cmd = handle_blocking_input();
        if cmd.move_cursor.is_some() {
            move_cursor(&mut asset, &cmd.move_cursor.unwrap());
        }
        if cmd.quit {
            break;
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
                x: line_idx,
                y: glyph_idx,
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

        _ => (),
    };
    return command;
}

fn move_cursor(asset: &mut Asset, direction: &Direction) {
    match direction {
        Direction::Left => {
            if asset.cursor_position.y > 1 {
                asset.cursor_position.y -= 1;
            }
        }
        Direction::Right => {
            if asset.cursor_position.y < asset.size.width {
                asset.cursor_position.y += 1;
            }
        }
        Direction::Up => {
            if asset.cursor_position.x > 1 {
                asset.cursor_position.x -= 1;
            }
        }
        Direction::Down => {
            if asset.cursor_position.x < asset.size.width {
                asset.cursor_position.x += 1;
            }
        }
    }
}

fn resize(asset: &mut Asset, direction: &Direction, delta: i32) {
    if delta > 0 {
        match direction {
            Direction::Left => {
                for frame_idx in 0..asset.animation.len() {
                    for line_idx in 0..asset.animation[frame_idx].len() {
                        asset.animation[frame_idx][line_idx].insert(0, EMPTY_COLOR_GLYPH);
                    }
                }
            }
            Direction::Right => {
                for frame_idx in 0..asset.animation.len() {
                    for line_idx in 0..asset.animation[frame_idx].len() {
                        asset.animation[frame_idx][line_idx].push(EMPTY_COLOR_GLYPH);
                    }
                }
            }
            Direction::Up => {
                for frame_idx in 0..asset.animation.len() {
                    asset.animation[frame_idx].insert(0, vec![]);
                    for _line_idx in 0..asset.animation[frame_idx][1].len() {
                        asset.animation[frame_idx][0].push(EMPTY_COLOR_GLYPH);
                    }
                }
            }
            Direction::Down => {
                for frame_idx in 0..asset.animation.len() {
                    asset.animation[frame_idx].push(vec![]);
                    for _line_idx in 0..asset.animation[frame_idx][1].len() {
                        asset.animation[frame_idx][0].push(EMPTY_COLOR_GLYPH);
                    }
                }
            }
        }
    } else if delta < 0 {
    }
}
