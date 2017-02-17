use spaceship::*;
use std::collections::HashMap;

pub struct StarSystem<'a> {
    pub id: i32,
    pub display_name: String,
    pub cargo: Vec<CargoInstance<'a>>,
    pub connected_to: Vec<i32>,
}

fn connect_star_systems(first_system: &mut StarSystem, second_system: &mut StarSystem) {
    first_system.connected_to.push(second_system.id);
    second_system.connected_to.push(first_system.id);
}

impl<'a> HoldsCargo<'a> for StarSystem<'a> {
    fn add_cargo(&mut self, cargo_type: &'a CargoItem, count: u32, value: f64) {
        let found_index =
            self.cargo.iter().position(|ref r| r.cargo.display_name == cargo_type.display_name);
        match found_index {
            Some(index) => self.cargo[index].count += count,
            None => self.cargo.push(CargoInstance::new(cargo_type, count, value)),
        }
    }

    fn remove_cargo(&mut self, cargo_type: &CargoItem, count: u32) {
        let found_index =
            self.cargo.iter().position(|ref r| r.cargo.display_name == cargo_type.display_name);
        match found_index {
            Some(x) => self.cargo[x].count = self.cargo[x].count - count,
            None => {}
        }
    }
}

pub struct GameDataLibrary<'a> {
    ships: HashMap<i32, Spaceship>,
    cargo: HashMap<i32, CargoItem>,
    systems: HashMap<i32, StarSystem<'a>>,
}

impl<'a> GameDataLibrary<'a> {
    pub fn new() -> GameDataLibrary<'a> {
        let mut a = GameDataLibrary {
            ships: HashMap::new(),
            cargo: HashMap::new(),
            systems: HashMap::new(),
        };

        let default_ship = Spaceship {
            display_name: "Ship".to_string(),
            fuel_capacity: 10,
            jump_distance: 100,
            cargo_capacity: 10u32,
        };

        a.ships.insert(0, default_ship);

        let default_cargo = CargoItem { display_name: "Cargo".to_string() };

        a.cargo.insert(0, default_cargo);

        let mut default_starsystem = StarSystem {
            id: 0,
            display_name: "Sol".to_string(),
            cargo: Vec::new(),
            connected_to: Vec::new(),
        };

        let mut second_starsystem = StarSystem {
            id: 1,
            display_name: "Alpha Centauri".to_string(),
            cargo: Vec::new(),
            connected_to: Vec::new(),
        };

        connect_star_systems(&mut default_starsystem, &mut second_starsystem);

        let system_id = default_starsystem.id;
        a.systems.insert(system_id, default_starsystem);
        let system_id = second_starsystem.id;
        a.systems.insert(system_id, second_starsystem);
        a
    }

    pub fn get_ship(&self, id: i32) -> &Spaceship {
        match self.ships.get(&id) {
            Some(ship) => ship,
            None => panic!("Couldn't find ship {}", id),
        }
    }

    pub fn get_cargo(&self, id: i32) -> &CargoItem {
        match self.cargo.get(&id) {
            Some(c) => c,
            None => panic!("Couldn't find the cargo {}", id),
        }
    }

    pub fn get_system(&self, id: i32) -> &StarSystem {
        match self.systems.get(&id) {
            Some(system) => system,
            None => panic!("Couldn't find the system {}", id),
        }
    }
}
