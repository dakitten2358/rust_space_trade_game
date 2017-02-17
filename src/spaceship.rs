pub trait HasCargoSpace {
    fn used_cargo_space(&self) -> u32;
}

pub trait HoldsCargo<'a> {
    fn add_cargo(&mut self, cargo_type: &'a CargoItem, count: u32, value: f64);
    fn remove_cargo(&mut self, cargo_type: &'a CargoItem, count: u32);
}

pub struct CargoItem {
    pub display_name: String,
}

impl HasCargoSpace for CargoItem {
    fn used_cargo_space(&self) -> u32 {
        1u32
    }
}

pub struct CargoInstance<'a> {
    pub cargo: &'a CargoItem,
    pub count: u32,
    pub value: f64,
}

impl<'a> HasCargoSpace for CargoInstance<'a> {
    fn used_cargo_space(&self) -> u32 {
        self.count * self.cargo.used_cargo_space()
    }
}

impl<'a> CargoInstance<'a> {
    pub fn new(cargo_type: &CargoItem, initial_count: u32, value_per_item: f64) -> CargoInstance {
        CargoInstance {
            cargo: cargo_type,
            count: initial_count,
            value: value_per_item,
        }
    }
}

pub struct Spaceship {
    pub display_name: String,
    pub fuel_capacity: u32,
    pub jump_distance: u32,
    pub cargo_capacity: u32,
}
