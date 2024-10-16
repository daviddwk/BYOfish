extern crate crossterm;
use terminal;

// I <3 tuples
// ===========
// 0 width
// 1 height

pub fn print_line(string: &str) {
    // right pad (^:
    terminal::set_background_color(terminal::Color::Default);
    print!("{}", string);
    new_line();
}

pub fn new_line() {
    terminal::set_background_color(terminal::Color::Default);
    if let Ok(size) = crossterm::terminal::size() {
        if let Ok(position) = crossterm::cursor::position() {
            let padding_len = (size.0 - position.0) as usize;
            print!("{}\n\r", " ".repeat(padding_len));
        }
    }
}

pub fn to_end() {
    terminal::set_background_color(terminal::Color::Default);
    if let Ok(size) = crossterm::terminal::size() {
        if let Ok(position) = crossterm::cursor::position() {
            let padding_len = (size.0 - position.0) as usize;
            print!("{}\n\r", " ".repeat(padding_len));

            for _i in 0..(size.1 - (position.1 + 2)) {
                print!("{}\n\r", " ".repeat(size.0 as usize));
                /*
                print!(
                    "height:{} - {} = {}\n\r",
                    size.1,
                    position.1 + 1,
                    size.1 - (position.1 + 1)
                );
                */
            }
        }
    }
}
