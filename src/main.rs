use std::io::stdout;
use std::process::exit;
extern crate structopt;
use std::path::PathBuf;
use structopt::StructOpt;

extern crate crossterm;
use crossterm::ExecutableCommand;
extern crate colored;
extern crate home;
extern crate rand;
extern crate serde_json;

mod mode;
use mode::EditorMode;
mod animation;
mod color_glyph;
mod error;
use error::error;
mod asset;
mod open_json;
use asset::Asset;
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

    let mut mode = EditorMode::Glyph;

    let asset_path: PathBuf = home::home_dir().unwrap().join("Documents");
    let mut asset = Asset::new(&asset_path, "test");
    // init terminal
    crossterm::terminal::enable_raw_mode().unwrap();
    stdout().execute(crossterm::cursor::Hide).unwrap();
    stdout()
        .execute(crossterm::terminal::DisableLineWrap)
        .unwrap();
    stdout()
        .execute(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::All,
        ))
        .unwrap();

    // main loop
    loop {
        asset.print();
        println!(
            "frame: {}, height:{} width:{}",
            asset.get_frame_idx(),
            asset.get_size().height,
            asset.get_size().width
        );
        println!(
            "x:{} y:{}",
            asset.get_cursor_position().x,
            asset.get_cursor_position().y
        );
        if mode == EditorMode::Glyph {
            println!("mode:glyph");
        } else {
            println!("mode:color");
        }
        let cmd = handle_blocking_input(&mode);
        if cmd.cycle_mode {
            if mode == EditorMode::Glyph {
                mode = EditorMode::Color;
            } else if mode == EditorMode::Color {
                mode = EditorMode::Glyph;
            }
        }
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
    stdout().execute(crossterm::cursor::Show).unwrap();
    crossterm::terminal::disable_raw_mode().unwrap();
    exit(0);
}
