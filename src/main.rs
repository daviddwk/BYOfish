extern crate structopt;
use structopt::StructOpt;

// get rid of this dep
extern crate serde_json;

mod animation;
mod color_glyph;
mod error;
mod terminal;
use error::error;
mod asset;
mod command;
mod decorations;
mod input;
mod menu;
mod mode;
mod open_json;

#[derive(Debug, structopt::StructOpt)]
#[structopt(
    name = "byofish",
    version = "0.0.1",
    about = "Create assets for freefish!"
)]
struct Opt {
    #[structopt(name = "FILE")]
    file_name: String,
}

fn main() {
    let args = Opt::from_args();
    let asset_path = std::path::PathBuf::from(args.file_name);

    let mut file_name = String::new();
    if let Some(dir) = asset_path.file_name() {
        // idk what I'm doing here, so there's probably a safer way
        file_name = dir.to_os_string().into_string().unwrap();
    }

    let mut asset = asset::Asset::new(&asset_path, &file_name);
    // TODO:look at exsisting file and make the type based on that or default to fish
    let mut save_menu = menu::SaveMenu::new(menu::AssetType::Fish(menu::FishSettings {}));

    let mut mode = mode::EditorMode::Glyph;
    terminal::init();

    loop {
        terminal::home_cursor();
        // if not save mode
        if mode == mode::EditorMode::Save {
            save_menu.print();
            if !save_menu.handle_blocking_input() {
                mode = mode::EditorMode::Glyph;
            }
        } else {
            decorations::print_frame_indicator(asset.get_frame_idx(), asset.get_frame_num());
            asset.print();
            decorations::print_color_guide();
            // else print save mode screen
            if mode == mode::EditorMode::Glyph {
                println!("\rmode:glyph");
            } else {
                println!("\rmode:color");
            }
            let cmd = command::handle_blocking_input(&mode);
            if cmd.quit {
                break;
            } else if cmd.cycle_mode {
                if mode == mode::EditorMode::Glyph {
                    mode = mode::EditorMode::Color;
                } else if mode == mode::EditorMode::Color {
                    mode = mode::EditorMode::Glyph;
                }
            } else if cmd.save_mode {
                mode = mode::EditorMode::Save;
            } else {
                asset.handle_command(&cmd);
            } // else if save mode
        }
    }
    // return terminal to regular state
    terminal::reset();
    std::process::exit(0);
}
