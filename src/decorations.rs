use crossterm::ExecutableCommand;
use std::io::stdout;

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

pub fn print_color_guide() {
    stdout()
        .execute(crossterm::style::SetForegroundColor(
            crossterm::style::Color::Black,
        ))
        .unwrap();

    stdout()
        .execute(crossterm::style::SetBackgroundColor(
            crossterm::style::Color::DarkGrey,
        ))
        .unwrap();
    print!("a");
    stdout()
        .execute(crossterm::style::SetBackgroundColor(
            crossterm::style::Color::Red,
        ))
        .unwrap();
    print!("r");
    stdout()
        .execute(crossterm::style::SetBackgroundColor(
            crossterm::style::Color::Green,
        ))
        .unwrap();
    print!("g");
    stdout()
        .execute(crossterm::style::SetBackgroundColor(
            crossterm::style::Color::Yellow,
        ))
        .unwrap();
    print!("y");
    stdout()
        .execute(crossterm::style::SetBackgroundColor(
            crossterm::style::Color::Blue,
        ))
        .unwrap();
    print!("b");
    stdout()
        .execute(crossterm::style::SetBackgroundColor(
            crossterm::style::Color::Magenta,
        ))
        .unwrap();
    print!("m");
    stdout()
        .execute(crossterm::style::SetBackgroundColor(
            crossterm::style::Color::Cyan,
        ))
        .unwrap();
    print!("c");
    stdout()
        .execute(crossterm::style::SetBackgroundColor(
            crossterm::style::Color::White,
        ))
        .unwrap();
    print!("w");
    print!("\n");

    stdout()
        .execute(crossterm::style::SetForegroundColor(
            crossterm::style::Color::White,
        ))
        .unwrap();
    print!("\r");

    stdout()
        .execute(crossterm::style::SetBackgroundColor(
            crossterm::style::Color::Black,
        ))
        .unwrap();
    print!("A");
    stdout()
        .execute(crossterm::style::SetBackgroundColor(
            crossterm::style::Color::DarkRed,
        ))
        .unwrap();
    print!("R");
    stdout()
        .execute(crossterm::style::SetBackgroundColor(
            crossterm::style::Color::DarkGreen,
        ))
        .unwrap();
    print!("G");
    stdout()
        .execute(crossterm::style::SetBackgroundColor(
            crossterm::style::Color::DarkYellow,
        ))
        .unwrap();
    print!("Y");
    stdout()
        .execute(crossterm::style::SetBackgroundColor(
            crossterm::style::Color::DarkBlue,
        ))
        .unwrap();
    print!("B");
    stdout()
        .execute(crossterm::style::SetBackgroundColor(
            crossterm::style::Color::DarkMagenta,
        ))
        .unwrap();
    print!("M");
    stdout()
        .execute(crossterm::style::SetBackgroundColor(
            crossterm::style::Color::DarkCyan,
        ))
        .unwrap();
    print!("C");
    stdout()
        .execute(crossterm::style::SetBackgroundColor(
            crossterm::style::Color::Grey,
        ))
        .unwrap();
    print!("W");
    stdout()
        .execute(crossterm::style::SetBackgroundColor(
            crossterm::style::Color::Reset,
        ))
        .unwrap();
    stdout()
        .execute(crossterm::style::SetForegroundColor(
            crossterm::style::Color::Reset,
        ))
        .unwrap();

    print!("\n\r");
}
