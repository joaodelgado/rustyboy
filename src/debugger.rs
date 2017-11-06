use cpu::Cpu;
use errors::{Error, ErrorKind, Result};

use std::io;

/// Debug commands
// Print stack's current state
const DEBUG_PRINT_MEM: &'static str = "s";
// Print cpu's current state
const DEBUG_PRINT_CPU: &'static str = "cpu";
// Revert to previous game boy's state
const DEBUG_BACKTRACK: &'static str = "p";

pub struct Debugger {
    previous_state: Cpu,
}

impl Debugger {
    pub fn new() -> Debugger {
        Debugger {
            previous_state: Cpu::new(),
        }
    }

    pub fn tick(&mut self, cpu: &mut Cpu) -> Result<()> {
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd)?;
        let cmd = cmd.trim();
        match cmd {
            DEBUG_BACKTRACK => {
                cpu.load_from(&self.previous_state);
                println!("{}", cpu);
            }
            DEBUG_PRINT_CPU => println!("{}", cpu),
            DEBUG_PRINT_MEM => {
                let mut range = String::new();
                io::stdin().read_line(&mut range).expect("read error");
                let range = range.trim();
                let vec = range.split_whitespace()
                    .map(|x| x.parse::<usize>().expect("parse error"))
                    .collect::<Vec<usize>>();

                let mut i = vec[0];
                for &n in cpu.get_mem_range(vec[0], vec[1]) {
                    println!("{}: {:X}", i, n);
                    i += 1;
                }
            }
            _ => {
                self.previous_state = cpu.clone();
                cpu.tick()?;
            }
        }
        Ok(())
    }
}
