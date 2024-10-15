use error;
use std::fs::File;
use std::path::PathBuf;

pub fn open_json(path: &PathBuf, name: &str, asset_type: &str) -> serde_json::Value {
    match File::open(path) {
        Ok(f) => match serde_json::from_reader(f) {
            Ok(j) => return j,
            Err(_e) => error(
                &format!("{} file {} is not proper json", asset_type, name),
                1,
            )
            .into(),
        },
        Err(_e) => error(&format!("could not open {} file {}", asset_type, name), 1).into(),
    }
}
pub fn format_json(json: &serde_json::Value) -> String {
    let json_string = serde_json::to_string(&json).unwrap();
    let json_chars = json_string.chars();
    let mut json_formatted: String = String::new();

    let mut prev_c = ' ';
    let mut in_quotes = false;
    let mut depth: usize = 0;

    // this is not ideal but it does line things up
    for c in json_chars {
        if c == '"' && prev_c != '\\' {
            in_quotes = !in_quotes;
        }

        json_formatted.push(c);

        if !in_quotes && (c == '{' || c == '[') {
            depth += 1;
            json_formatted.push('\n');
            for _i in 0..depth {
                json_formatted.push(' ');
            }
        } else if !in_quotes && (c == '}' || c == ']') {
            depth -= 1;
            json_formatted.push('\n');
            for _i in 0..depth {
                json_formatted.push(' ');
            }
        }

        if !in_quotes && (c == ',' && prev_c == '"') {
            json_formatted.push('\n');
            for _i in 0..depth {
                json_formatted.push(' ');
            }
        }

        prev_c = c;
    }

    return json_formatted;
}
