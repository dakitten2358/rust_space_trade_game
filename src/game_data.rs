use spaceship::*;
use std::collections::HashMap;

pub struct GameDataLibrary {
    ships: HashMap<i32, Spaceship>,
    cargo: HashMap<i32, CargoItem>,
}

impl GameDataLibrary {
    pub fn new() -> GameDataLibrary {
        let mut a = GameDataLibrary {
            ships: HashMap::new(),
            cargo: HashMap::new(),
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
}
