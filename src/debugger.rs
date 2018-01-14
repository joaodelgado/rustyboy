use cpu::Cpu;
use std::io::stdout;
use errors::Result;

use std::io;
use std::io::Write;

/// Debug commands
// Step command
const DEBUG_STEP: &str = "";
// Print stack's current state
const DEBUG_PRINT_MEM: &str = "s";
// Print cpu's current state
const DEBUG_PRINT_CPU: &str = "cpu";
// Revert to previous game boy's state
const DEBUG_BACKTRACK: &str = "p";
// Step n times
const DEBUG_UNTIL_N: &str = "n";

pub struct Debugger {
    previous_state: Cpu,
    n_iteration: u64,
}

impl Debugger {
    pub fn new() -> Debugger {
        Debugger {
            previous_state: Cpu::new(),
            n_iteration: 0,
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
        if vec.is_empty() {
            vec.push(DEBUG_STEP.to_string());
        }

        vec
    }

    fn step(&mut self, cpu: &mut Cpu) -> Result<()> {
        self.previous_state = cpu.clone();
        cpu.tick()?;
        self.n_iteration += 1;
        Ok(())
    }

    pub fn tick(&mut self, cpu: &mut Cpu) -> Result<()> {
        print!("rustyboy({})> ", self.n_iteration);
        stdout().flush()?;

        let cmd = self.read_line();
        match cmd[0].as_str() {
            DEBUG_BACKTRACK => {
                cpu.load_from(&self.previous_state);
                self.n_iteration -= 1;
                println!("{}", cpu);
            }
            DEBUG_PRINT_CPU => println!("{}", cpu),
            DEBUG_PRINT_MEM => {
                let args = &cmd[1..3]
                    .iter()
                    .map(|x| x.parse::<u16>().expect("parse error"))
                    .collect::<Vec<u16>>();

                for &n in cpu.get_mem_range(args[0] as usize, args[1] as usize) {
                    println!("{:02x}", n);
                }
            }
            DEBUG_UNTIL_N => {
                let pc_val = &cmd[1..2]
                    .iter()
                    .map(|x| x.parse::<u16>().expect("parse error"))
                    .collect::<Vec<u16>>()
                    [0];

                for _ in 0..pc_val + 1 {
                    self.step(cpu)?;
                }
            }
            _ => {
                self.step(cpu)?;
            }
        }
        Ok(())
    }
}
