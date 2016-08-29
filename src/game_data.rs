use spaceship::*;
use std::collections::HashMap;

pub struct GameDataLibrary {
	ships : HashMap<i32, Spaceship>
}

impl GameDataLibrary {
	pub fn new() -> GameDataLibrary {
		let mut a = GameDataLibrary { ships: HashMap::new() };

		let default_ship = Spaceship {
	        display_name: "Ship".to_string(),
	        fuel_capacity: 10,
	        jump_distance: 100,
	        cargo_capacity: 10,
	    };

		a.ships.insert(0, default_ship);
		a
	}

	pub fn get_ship(&self, id : i32) -> &Spaceship {
    	match self.ships.get(&id) {
    		Some(ship) => ship,
    		None => panic!("Couldn't find ship {}", id),
    	}
    }
}
