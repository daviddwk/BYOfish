use input;
use pad;

pub enum AssetType {
    Fish,
    Duck,
    Crab,
}

pub struct FishSettings {}
const DUCK_NUM_SETTINGS: usize = 2;
pub struct DuckSettings {
    buoyancy: isize,
}
pub struct CrabSettings {}

pub struct SaveMenu {
    sort: AssetType,
    cursor_idx: usize,
    fish_settings: FishSettings,
    duck_settings: DuckSettings,
    crab_settings: CrabSettings,
}

impl SaveMenu {
    pub fn new(sort: AssetType) -> SaveMenu {
        return SaveMenu {
            sort,
            cursor_idx: 0,
            fish_settings: FishSettings {},
            duck_settings: DuckSettings { buoyancy: 0 },
            crab_settings: CrabSettings {},
        };
    }

    pub fn print(&self) {
        match &self.sort {
            AssetType::Fish => pad::print_line("Asset Type: < Fish > "),
            AssetType::Duck => self.print_duck_settings(),
            AssetType::Crab => pad::print_line("Asset Type: < Crab > "),
        }
    }

    fn print_duck_settings(&self) {
        let duck_menu_text = [
            "Asset Type: < Duck > ".to_string(),
            format!("Buoyancy: [{}]", self.duck_settings.buoyancy),
        ];
        print_settings(self.cursor_idx, &duck_menu_text);
    }

    pub fn handle_input(&mut self) -> bool {
        if let Some(press) = input::get_press() {
            if press.key == input::Key::Esc {
                return false;
            } else if press.key == input::Key::Direction(input::Direction::Up) {
                if self.cursor_idx > 0 {
                    self.cursor_idx -= 1;
                }
            } else if press.key == input::Key::Direction(input::Direction::Down) {
                if self.cursor_idx < (DUCK_NUM_SETTINGS - 1) {
                    self.cursor_idx += 1;
                }
            }

            if self.cursor_idx == 0 {
                if let input::Key::Direction(dir) = press.key {
                    if dir == input::Direction::Left {
                        self.cycle_asset_type(-1);
                    } else if dir == input::Direction::Right {
                        self.cycle_asset_type(1);
                    }
                }
            }
            match self.sort {
                AssetType::Fish => {}
                AssetType::Duck => match self.cursor_idx {
                    1 => {
                        self.duck_settings.buoyancy =
                            handle_input_usize(self.duck_settings.buoyancy, press.key);
                    }
                    _ => {}
                },
                AssetType::Crab => {}
            }
        }
        return true;
    }

    fn cycle_asset_type(&mut self, delta: isize) {
        // just use a match
        if delta > 0 {
            for _i in 0..delta {
                match self.sort {
                    AssetType::Fish => {
                        let new_type = AssetType::Duck;
                        self.sort = new_type;
                    }
                    AssetType::Duck => {
                        let new_type = AssetType::Crab;
                        self.sort = new_type;
                    }
                    AssetType::Crab => {
                        let new_type = AssetType::Fish;
                        self.sort = new_type;
                    }
                }
            }
        } else {
            for _i in 0..-delta {
                match self.sort {
                    AssetType::Fish => {
                        let new_type = AssetType::Crab;
                        self.sort = new_type;
                    }
                    AssetType::Duck => {
                        let new_type = AssetType::Fish;
                        self.sort = new_type;
                    }
                    AssetType::Crab => {
                        let new_type = AssetType::Duck;
                        self.sort = new_type;
                    }
                }
            }
        }
    }
}

fn handle_input_usize(num: isize, key: input::Key) -> isize {
    match key {
        input::Key::Direction(input::Direction::Left) => {
            return num - 1;
        }
        input::Key::Direction(input::Direction::Right) => {
            return num + 1;
        }
        _ => return num,
    }
}

fn print_settings(cursor_idx: usize, menu_text: &[String]) {
    for setting_idx in 0..menu_text.len() {
        let mut string: String = menu_text[setting_idx].clone();
        if setting_idx == cursor_idx {
            string = format!("→ {}", string);
        } else {
            string = format!("  {}", string);
        }
        pad::print_line(&string);
    }
}
