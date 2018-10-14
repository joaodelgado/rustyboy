extern crate rustyboy;

use std::env;
use std::process;

use rustyboy::game_boy::GameBoy;
use rustyboy::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });

    let mut game_boy = GameBoy::new(&config).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });

    game_boy.run().unwrap_or_else(|e| {
        eprintln!("Unexpected error while running: {}", e);
        process::exit(1);
    });
}
