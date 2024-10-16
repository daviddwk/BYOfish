use input;
use pad;

pub struct FishSettings {}

pub struct DuckSettings {
    buoyancy: usize,
}

pub struct CrabSettings {}

pub enum AssetType {
    Fish(FishSettings),
    Duck(DuckSettings),
    Crab(CrabSettings),
}

pub struct SaveMenu {
    sort: AssetType,
}

impl SaveMenu {
    pub fn new(sort: AssetType) -> SaveMenu {
        return SaveMenu { sort };
    }

    pub fn print(&self) {
        match &self.sort {
            AssetType::Fish(_settings) => pad::print_line("Asset Type: < Fish > "),
            AssetType::Duck(settings) => {
                pad::print_line("Asset Type: < Duck > ");
                pad::print_line(&format!("\rBuoyancy: [{}]", settings.buoyancy));
            }
            AssetType::Crab(_settings) => pad::print_line("Asset Type: < Crab > "),
        }
    }

    pub fn handle_blocking_input(&mut self) -> bool {
        let press = input::blocking_get_press();
        if press.key == input::Key::Esc {
            return false;
        }
        if let input::Key::Direction(dir) = press.key {
            if dir == input::Direction::Left {
                self.cycle_asset_type(-1);
            } else if dir == input::Direction::Right {
                self.cycle_asset_type(1);
            }
        }
        if let AssetType::Duck(ref mut settings) = self.sort {
            if let input::Key::Glyph(glyph) = press.key {
                if let Some(num) = glyph.to_digit(10) {
                    settings.buoyancy = num as usize;
                }
            }
        }
        return true;
    }

    fn cycle_asset_type(&mut self, delta: isize) {
        // just use a match
        if delta > 0 {
            for _i in 0..delta {
                match self.sort {
                    AssetType::Fish(..) => {
                        let new_type = AssetType::Duck(DuckSettings { buoyancy: 0 });
                        self.sort = new_type;
                    }
                    AssetType::Duck(..) => {
                        let new_type = AssetType::Crab(CrabSettings {});
                        self.sort = new_type;
                    }
                    AssetType::Crab(..) => {
                        let new_type = AssetType::Fish(FishSettings {});
                        self.sort = new_type;
                    }
                }
            }
        } else {
            for _i in 0..-delta {
                match self.sort {
                    AssetType::Fish(..) => {
                        let new_type = AssetType::Crab(CrabSettings {});
                        self.sort = new_type;
                    }
                    AssetType::Duck(..) => {
                        let new_type = AssetType::Fish(FishSettings {});
                        self.sort = new_type;
                    }
                    AssetType::Crab(..) => {
                        let new_type = AssetType::Duck(DuckSettings { buoyancy: 0 });
                        self.sort = new_type;
                    }
                }
            }
        }
    }
}
