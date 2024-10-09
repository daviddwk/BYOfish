use std::io::stdout;
use std::process::exit;
extern crate structopt;
use colored::Colorize;
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
mod asset;
mod open_json;
use asset::Asset;
mod commands;
use commands::{Command, Direction};
mod input;
use input::handle_blocking_input;

#[derive(StructOpt)]
#[structopt(
    name = "byofish",
    version = "0.0.1",
    about = "Create assets for freefish!"
)]
struct Opt {}

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
            commands::move_cursor(&mut asset, &mv);
        }
        if cmd.quit {
            break;
        }
        if let Some(rz) = cmd.resize {
            commands::resize(&mut asset, &rz.0, rz.1);
        }
        if let Some(glyph) = cmd.set_char {
            commands::set_char(&mut asset, glyph);
        }
        if let Some(color) = cmd.set_color {
            commands::set_color(&mut asset, color);
        }
        if cmd.add_frame {
            commands::add_frame(&mut asset);
        }
        if cmd.delete_frame {
            commands::delete_frame(&mut asset);
        }
        if let Some(delta) = cmd.cycle_frame {
            commands::cycle_frame(&mut asset, delta);
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
    for line_idx in 0..asset.size.height {
        // print top line
        if line_idx == 0 {
            print!("┏{}┓ \r\n", "━".repeat(asset.size.width));
        }
        for glyph_idx in 0..asset.size.width {
            if glyph_idx == 0 {
                print!("┃");
            }
            let pos = Position {
                x: glyph_idx,
                y: line_idx,
            };
            // TODO make cursor flashing
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
            if glyph_idx == asset.size.width - 1 {
                print!("┃ ");
            }
        }
        print!("\r\n");
        // print bottom line
        if line_idx == asset.size.height - 1 {
            print!("┗{}┛ \r\n", "━".repeat(asset.size.width));
        }
    }
}
