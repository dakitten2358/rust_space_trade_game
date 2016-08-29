pub trait HasCargoSpace {
    fn used_cargo_space(&self) -> u32;
}

pub struct CargoItem {
    pub display_name: String,
}

impl HasCargoSpace for CargoItem {
    fn used_cargo_space(&self) -> u32 {
        1u32
    }
}
pub struct Spaceship {
    pub display_name: String,
    pub fuel_capacity: u32,
    pub jump_distance: u32,
    pub cargo_capacity: u32,
}
