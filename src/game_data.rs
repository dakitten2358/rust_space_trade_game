use spaceship::*;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::*;
use std::*;
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct StarSystem {
    pub id: i32,
    pub display_name: String,
    pub cargo: Vec<CargoInstance>,
    pub connected_to: Vec<i32>,
}

fn connect_star_systems(first_system: &mut StarSystem, second_system: &mut StarSystem) {
    first_system.connected_to.push(second_system.id);
    second_system.connected_to.push(first_system.id);
}

impl HoldsCargo for StarSystem {
    fn add_cargo(&mut self, cargo_type: &CargoItem, count: u32, value: f64) {
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

#[derive(Serialize, Deserialize)]
pub struct GameDataLibrary {
    ships: HashMap<i32, Spaceship>,
    cargo: HashMap<i32, CargoItem>,
    systems: HashMap<i32, StarSystem>,
}

impl GameDataLibrary {
    pub fn new() -> GameDataLibrary {

        let mut a = GameDataLibrary {
            ships: HashMap::new(),
            cargo: HashMap::new(),
            systems: HashMap::new(),
        };
        // let default_ship = Spaceship {
        // display_name: "Ship".to_string(),
        // fuel_capacity: 10,
        // jump_distance: 100,
        // cargo_capacity: 10u32,
        // };
        //
        // a.ships.insert(0, default_ship);
        //
        //
        // a.load_ships_from_json("jason.json");
        //
        //
        // let default_cargo = CargoItem { display_name: "Cargo".to_string() };
        //
        // a.cargo.insert(0, default_cargo.clone());
        //
        // let mut default_starsystem = StarSystem {
        // id: 0,
        // display_name: "Sol".to_string(),
        // cargo: Vec::new(),
        // connected_to: Vec::new(),
        // };
        //
        // default_starsystem.add_cargo(&default_cargo, 12, 3.1459);
        //
        // let mut second_starsystem = StarSystem {
        // id: 1,
        // display_name: "Alpha Centauri".to_string(),
        // cargo: Vec::new(),
        // connected_to: Vec::new(),
        // };
        //
        // connect_star_systems(&mut default_starsystem, &mut second_starsystem);
        //
        // let system_id = default_starsystem.id;
        // a.systems.insert(system_id, default_starsystem);
        // let system_id = second_starsystem.id;
        // a.systems.insert(system_id, second_starsystem);
        //
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

    pub fn load_json(&mut self, file_name: &str) -> io::Result<()> {
        let mut f = try!(File::open("data/game_library.json"));
        let mut buffer = String::new();
        try!(f.read_to_string(&mut buffer));
        println!("{}", buffer);
        let gl: GameDataLibrary = serde_json::from_str(&buffer).unwrap();

        for (id, ship) in &gl.ships {
            self.ships.insert(*id, ship.clone());
        }

        for (id, cargo) in &gl.cargo {
            self.cargo.insert(*id, cargo.clone());
        }

        for (id, system) in &gl.systems {
            self.systems.insert(*id, system.clone());
        }

        Ok(())
    }
}
