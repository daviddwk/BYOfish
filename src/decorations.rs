use terminal::{set_background_color, set_foreground_color, Color};

pub fn print_frame_indicator(frame_idx: usize, frame_num: usize) {
    for idx in 0..frame_num {
        if idx == frame_idx {
            print!("{}[*] ", idx);
        } else {
            print!("{}[ ] ", idx);
        }
    }
    print!("\n\r");
}

// make a set fg color function
// and maybe even wrap crossterm::Color
// to decouple dependency bc why not idk
pub fn print_color_guide() {
    print!("\r");
    set_foreground_color(Color::Black);

    set_background_color(Color::DarkGrey);
    print!("a");
    set_background_color(Color::Red);
    print!("r");
    set_background_color(Color::Green);
    print!("g");
    set_background_color(Color::Yellow);
    print!("y");
    set_background_color(Color::Blue);
    print!("b");
    set_background_color(Color::Magenta);
    print!("m");
    set_background_color(Color::Cyan);
    print!("c");
    set_background_color(Color::White);
    print!("w");

    print!("\n\r");
    set_foreground_color(Color::White);

    set_background_color(Color::Black);
    print!("a");
    set_background_color(Color::DarkRed);
    print!("r");
    set_background_color(Color::DarkGreen);
    print!("g");
    set_background_color(Color::DarkYellow);
    print!("y");
    set_background_color(Color::DarkBlue);
    print!("b");
    set_background_color(Color::DarkMagenta);
    print!("m");
    set_background_color(Color::DarkCyan);
    print!("c");
    set_background_color(Color::Grey);
    print!("w");

    set_foreground_color(Color::Default);
    set_background_color(Color::Default);

    print!("\n\r");
}
