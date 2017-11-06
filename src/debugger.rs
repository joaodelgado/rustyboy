use cpu::Cpu;
use std::io::{stdout};
use errors::{Result};

use std::io;
use std::io::Write;

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

    fn read_line(&self) -> Vec<String> {
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).expect("read error");
        let cmd = cmd.trim();

        let mut vec = cmd.split_whitespace()
            .map(|x| x.parse::<String>().expect("parse error"))
            .collect::<Vec<String>>();

        // step command
        if vec.len() == 0 {
            vec.push(String::new());
        }

        vec
    }

    pub fn tick(&mut self, cpu: &mut Cpu) -> Result<()> {
        print!("rustyboy> ");
        stdout().flush()?;

        let cmd = self.read_line();
        match cmd[0].as_str() {
            DEBUG_BACKTRACK => {
                cpu.load_from(&self.previous_state);
                println!("{}", cpu);
            }
            DEBUG_PRINT_CPU => println!("{}", cpu),
            DEBUG_PRINT_MEM => {
                let args = &cmd[1..3].iter()
                    .map(|x| x.parse::<u16>().expect("parse error"))
                    .collect::<Vec<u16>>();

                println!("{:?}", cpu.get_mem_range(args[0] as usize, args[1] as usize));
            }
            _ => {
                self.previous_state = cpu.clone();
                cpu.tick()?;
            }
        }
        Ok(())
    }
}
