mod game_data;
mod spaceship;

use spaceship::*;
use std::io;

enum PlayerCommand {
    Quit,
    Help,
    JumpToStarSystem { system_id: i32 },
    UnknownAction,
}

struct PlayerSpaceship<'a, 'b> {
    base: &'a Spaceship,
    current_fuel: u32,
    cargo: Vec<&'b CargoItem>,
}

impl<'a, 'b> HasCargoSpace for PlayerSpaceship<'a, 'b> {
    fn used_cargo_space(&self) -> u32 {
        self.cargo.iter().fold(0u32, |sum, x| sum + x.used_cargo_space())
    }
}

fn main() {

    let lib = game_data::GameDataLibrary::new();
    let c = CargoItem { display_name: "test".to_string() };

    let mut player_ship = PlayerSpaceship {
        base: lib.get_ship(0),
        current_fuel: lib.get_ship(0).fuel_capacity,
        cargo: Vec::new(),
    };

    player_ship.cargo.push(&c);

    loop {
        print_ship_status(&player_ship);

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
                println!("jumping to: {}", system_id);
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
        "quit" => PlayerCommand::Quit,
        "help" => PlayerCommand::Help,
        "jump" => PlayerCommand::JumpToStarSystem { system_id: 0 },
        _ => PlayerCommand::UnknownAction,
    }
}

fn print_ship_status(ship: &PlayerSpaceship) {
    println!("You are flying a {}, and your fuel is at {}/{}",
             ship.base.display_name,
             ship.current_fuel,
             ship.base.fuel_capacity);

    print!("Your cargo consumes {} of {} spaces",
           ship.used_cargo_space(),
           ship.base.cargo_capacity);
}
