use std::num::ParseIntError;

use crate::vm::VM;

pub struct Repl {
    commands: Vec<String>,
    vm: VM,
}

impl Repl {
    pub fn new() -> Repl {
        Repl {
            vm: VM::new(),
            commands: vec![],
        }
    }

    pub fn run_hex_program(&mut self, hex_program: Vec<&str>) {
        hex_program.iter().for_each(|command| {
            self.execute_hex_command(&command);
        });
    }

    pub fn execute_hex_command(&mut self, command: &str) {
        self.commands.push(command.to_string());
        match command {
            ".program" => {
                println!("Listing instructions currently in VM's program vector:");
                for instruction in &self.vm.program {
                    println!("{}", instruction);
                }
                println!("End of Program Listing");
            }
            ".commands" => {
                for command in &self.commands {
                    println!("{}", command);
                }
            }
            ".registers" => {
                println!("Listing registers and all contents:");
                println!("{:#?}", self.vm.registers);
                println!("End of Register Listing")
            }
            ".quit" => {
                println!("Farewell! Have a great day!");
                std::process::exit(0);
            }
            _ => {
                let results = self.parse_hex(command);
                match results {
                    Ok(bytes) => {
                        for byte in bytes {
                            self.vm.add_byte(byte)
                        }
                    }
                    Err(_e) => {
                        println!("Unable to decode hex string. Please enter 4 groups of 2 hex characters.")
                    }
                };
                self.vm.run_once();
            }
        }
    }

    /// Accepts a hexadecimal string WITHOUT a leading `0x` and returns a Vec of u8
    /// Example for a LOAD command: 00 01 03 E8
    fn parse_hex(&self, input: &str) -> Result<Vec<u8>, ParseIntError> {
        let mut results: Vec<u8> = vec![];

        let mut byte_results = input
            .split(" ")
            .map(|hex_str| u8::from_str_radix(&hex_str, 16));

        for byte_result in byte_results {
            match byte_result {
                Ok(result) => {
                    results.push(result);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
    }
}
