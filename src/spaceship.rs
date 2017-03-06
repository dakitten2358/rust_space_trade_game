pub trait HasCargoSpace {
    fn used_cargo_space(&self) -> u32;
}

pub trait HoldsCargo {
    fn add_cargo(&mut self, cargo_type: &CargoItem, count: u32, value: f64);
    fn remove_cargo(&mut self, cargo_type: &CargoItem, count: u32);
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CargoItem {
    pub display_name: String,
}

impl HasCargoSpace for CargoItem {
    fn used_cargo_space(&self) -> u32 {
        1u32
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CargoInstance {
    pub cargo: CargoItem,
    pub count: u32,
    pub value: f64,
}

impl HasCargoSpace for CargoInstance {
    fn used_cargo_space(&self) -> u32 {
        self.count * self.cargo.used_cargo_space()
    }
}

impl CargoInstance {
    pub fn new(cargo_type: &CargoItem, initial_count: u32, value_per_item: f64) -> CargoInstance {
        CargoInstance {
            cargo: cargo_type.clone(),
            count: initial_count,
            value: value_per_item,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Spaceship {
    pub display_name: String,
    pub fuel_capacity: u32,
    pub jump_distance: u32,
    pub cargo_capacity: u32,
}
