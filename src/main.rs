#[macro_use]
extern crate serde_derive;
extern crate serde_json;


mod game_data;
mod spaceship;

use spaceship::*;
use std::io;
use game_data::*;

enum PlayerCommand {
    Quit,
    Help,
    JumpToStarSystem { system_id: i32 },
    UnknownAction,
}

struct PlayerSpaceship<'a> {
    base: &'a Spaceship,
    current_fuel: u32,
    cargo: Vec<CargoInstance>,
}

impl<'a> HasCargoSpace for PlayerSpaceship<'a> {
    fn used_cargo_space(&self) -> u32 {
        self.cargo.iter().fold(0u32, |sum, x| sum + x.used_cargo_space())
    }
}

impl<'a> PlayerSpaceship<'a> {
    pub fn add_cargo(&mut self, cargo_type: &CargoItem, count: u32, value: f64) {
        self.cargo.push(CargoInstance::new(cargo_type, count, value));
    }
}

fn main() {

    let mut lib = game_data::GameDataLibrary::new();
    match lib.load_json("data/game_library.json") {
        Ok(_) => println!("data loaded.\n"),
        Err(_) => panic!("failed to load data"),
    }

    let mut player_ship = PlayerSpaceship {
        base: lib.get_ship(0),
        current_fuel: lib.get_ship(0).fuel_capacity,
        cargo: Vec::new(),
    };

    player_ship.add_cargo(lib.get_cargo(0), 3u32, 1.0f64);

    let mut current_system = lib.get_system(0);

    loop {
        print_ship_status(&player_ship);
        print_system_info(&lib, &current_system);

        // get player input
        let player_text = read_player_input();
        let player_input_tokens = tokenize_input(&player_text);

        // figure out which command we're running
        let player_command = map_input_tokens_to_command(&player_input_tokens);

        // do stuff
        match player_command {
            PlayerCommand::Quit => {
                break;
            }
            PlayerCommand::Help => {
                println!("ermg! halp!");
            }
            PlayerCommand::JumpToStarSystem { system_id } => {
                current_system = lib.get_system(system_id);
                println!("jumping to: {}", current_system.display_name);
            }
            _ => {
                println!("Unknown command!  Input \"help\" for a list of commands");
            }
        }
    }
}

fn read_player_input() -> String {
    let mut player_input = "".to_string();
    io::stdin().read_line(&mut player_input).expect("failed to read line");
    player_input
}

fn tokenize_input(player_input: &String) -> Vec<String> {
    player_input.to_lowercase()
        .as_str()
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

fn map_input_tokens_to_command(tokens: &Vec<String>) -> PlayerCommand {
    match tokens[0].as_str() {
        "q" => PlayerCommand::Quit,
        "quit" => PlayerCommand::Quit,
        "help" => PlayerCommand::Help,
        "jump" => {
            if tokens.len() > 1 {
                PlayerCommand::JumpToStarSystem { system_id: tokens[1].parse::<i32>().unwrap() }
            } else {
                PlayerCommand::UnknownAction
            }
        }
        _ => PlayerCommand::UnknownAction,
    }
}

fn print_ship_status(ship: &PlayerSpaceship) {
    println!("You are flying a {}, and your fuel is at {}/{}",
             ship.base.display_name,
             ship.current_fuel,
             ship.base.fuel_capacity);

    println!("Your cargo consumes {} of {} spaces.",
             ship.used_cargo_space(),
             ship.base.cargo_capacity);
}

fn print_system_info(lib: &GameDataLibrary, system: &StarSystem) {
    println!("You are in the {} system.  From here, you can jump to:",
             system.display_name);

    let ref connected_to = system.connected_to;
    for connected_system_id in connected_to {
        let connected_system = lib.get_system(*connected_system_id);

        println!("\t{}\t{}",
                 connected_system_id,
                 connected_system.display_name);
    }
}
