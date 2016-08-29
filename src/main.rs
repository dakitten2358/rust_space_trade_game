use std::io;
extern crate core;

enum PlayerCommand {
	Quit,
	Help,
	JumpToStarSystem { system_id : i32 },
	UnknownAction
}

fn main() {
    
    loop {
    	println!("Hello, world!");
    	
    	// get player input
    	let player_text = read_player_input();
    	let player_input_tokens = tokenize_input(&player_text);
    	
    	// figure out which command we're running
    	let player_command = map_input_tokens_to_command(&player_input_tokens);
    	
    	// do stuff
    	match player_command {
    		PlayerCommand::Quit => { break; },
    		PlayerCommand::Help => { println!("ermg! halp!"); },
    		PlayerCommand::JumpToStarSystem {system_id : system_id} => { println!("jumping to: {}", system_id); }
    		_ => { println!("Unknown command!  Input \"help\" for a list of commands");},
    	}
    }
}

fn read_player_input() -> String {
	let mut player_input = "".to_string();
	io::stdin().read_line(&mut player_input).expect("failed to read line");
	player_input
}

fn tokenize_input<'a>(player_input :&'a String) -> Vec<String> {
	player_input.to_lowercase().as_str().split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>()
}

fn map_input_tokens_to_command(tokens : &Vec<String>) -> PlayerCommand {
	match tokens[0].as_str() {
		"quit" => PlayerCommand::Quit,
    	"help" => PlayerCommand::Help,
    	"jump" => PlayerCommand::JumpToStarSystem {system_id: 0},
    	_ => PlayerCommand::UnknownAction,
   	}
}