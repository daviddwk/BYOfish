use std::io::stdout;
use std::process::exit;
extern crate structopt;
use structopt::StructOpt;

extern crate crossterm;
use crossterm::{
    cursor::{Hide, Show},
    terminal::{disable_raw_mode, enable_raw_mode, Clear},
    ExecutableCommand,
};
extern crate colored;
extern crate home;
extern crate rand;
extern crate serde_json;

mod animation;
use animation::{blank_animation, Position, Size};
mod color_glyph;
mod error;
use error::error;
mod asset;
mod open_json;
use asset::Asset;
mod commands;
use commands::Command;
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
    let _args = Opt::from_args();
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
        asset.print();
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
            asset.move_cursor(&mv);
        }
        if cmd.quit {
            break;
        }
        if let Some(rz) = cmd.resize {
            asset.resize(&rz.0, rz.1);
        }
        if let Some(glyph) = cmd.set_char {
            asset.set_char(glyph);
        }
        if let Some(color) = cmd.set_color {
            asset.set_color(color);
        }
        if cmd.add_frame {
            asset.add_frame();
        }
        if cmd.delete_frame {
            asset.delete_frame();
        }
        if let Some(delta) = cmd.cycle_frame {
            asset.cycle_frame(delta);
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
