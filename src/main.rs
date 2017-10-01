extern crate rustyboy;

use std::env;
use std::process;

use rustboy::Config;
use rustboy::utils::to_hex_string;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|e| {
        eprintln!("Error parsing arguments: {}", e);
        process::exit(1);
    });

    let data = rustboy::read_rom(&config).unwrap_or_else(|e| {
        eprintln!("Error reading rom: {}", e);
        process::exit(1);
    });

    println!("Read {} bytes", data.len());
    println!("First 50 bytes: {}", to_hex_string(&data[0x100..0x110]));
}
