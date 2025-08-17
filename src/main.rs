mod board;
mod mapper;

use board::Board;
use std::io::{stdin, stdout, Write};

use crate::mapper::str_to_coordinates;

fn options() -> Vec<String> {
    vec!["1. Attack".to_string(), "2. Exit".to_string()]
}

fn main() {
    let mut player1 = Board::new();
    let mut player2 = Board::new();
    let mut game_turn = true;
    player1.init_rnd();
    player2.init_rnd();

    let mut player_res = String::new();
    loop {
        println!("Board of player 1");
        player1.show();

        println!("Board of player 2");
        player2.show();

        println!();
        for opt in &options() {
            println!("{opt}");
        }
        print!("> ");
        stdout().flush().unwrap();
        player_res.clear();
        match stdin().read_line(&mut player_res) {
            Err(e) => println!("Failed to read: {e}"), 
            _ => ()
        }

        if player_res.trim() == "2" {
            break;
        }

        let mut attack = String::new();
        println!();
        if game_turn {
            print!("P1 Attack:");
            stdout().flush().unwrap();
            player_res.clear();
            match stdin().read_line(&mut attack) { 
                Err(e) => print!("Failed to read attack: {e}"), 
                _ => ()
            }
            let coordinates = str_to_coordinates(&attack);
            println!("{coordinates:?}");
        } else {
            print!("P2 Attack:");
            stdout().flush().unwrap();
            player_res.clear();
            match stdin().read_line(&mut attack) {
                Err(e) => print!("Failed to read attack: {e}"), 
                _ => ()
            }
            let coordinates = str_to_coordinates(&attack);
            println!("{coordinates:?}");
        }
        println!();
        game_turn = !game_turn
    }
}
