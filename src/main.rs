use std::process::exit;
extern crate structopt;
use std::path::PathBuf;
use structopt::StructOpt;

extern crate colored;
extern crate home;
extern crate rand;
extern crate serde_json;

mod mode;
mod terminal;
use mode::EditorMode;
mod animation;
mod color_glyph;
mod error;
use error::error;
mod asset;
mod open_json;
use asset::Asset;
mod command;
mod decorations;
mod input;
use decorations::{print_color_guide, print_frame_indicator};

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

    // main loop
    terminal::init();
    loop {
        terminal::home_cursor();
        print_frame_indicator(asset.get_frame_idx(), asset.get_frame_num());
        asset.print();
        print_color_guide();
        println!(
            "\rframe: {} / {}\n\rheight:{} width:{}",
            asset.get_frame_idx(),
            asset.get_frame_num(),
            asset.get_size().height,
            asset.get_size().width
        );
        if mode == EditorMode::Glyph {
            println!("\rmode:glyph");
        } else {
            println!("\rmode:color");
        }
        let cmd = command::handle_blocking_input(&mode);
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
            asset.set_color(&color);
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
    terminal::reset();
    exit(0);
}
