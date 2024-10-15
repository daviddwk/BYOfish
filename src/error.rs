extern crate crossterm;

pub fn error(msg: &str, code: i32) {
    crossterm::style::SetForegroundColor(crossterm::style::Color::Red);
    crossterm::style::SetBackgroundColor(crossterm::style::Color::Reset);
    crossterm::style::SetAttribute(crossterm::style::Attribute::Bold);
    print!("error:");
    crossterm::style::SetForegroundColor(crossterm::style::Color::Reset);
    crossterm::style::SetAttribute(crossterm::style::Attribute::Reset);
    println!("{}", msg);
    std::process::exit(code);
}
