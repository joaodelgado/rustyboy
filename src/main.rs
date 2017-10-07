extern crate rustyboy;

use std::env;
use std::process;

use rustyboy::Config;
use rustyboy::game_boy::GameBoy;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|e| {
        eprintln!("Error parsing arguments: {}", e);
        process::exit(1);
    });

    let game_boy = GameBoy::new(&config).unwrap_or_else(|e| {
        eprintln!("Error reading rom: {}", e);
        process::exit(1);
    });

    println!("Read rom with title: {}", game_boy.cartridge.title());
}
