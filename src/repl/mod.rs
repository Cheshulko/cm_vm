use std::num::ParseIntError;

use log::{debug, error, info};

use crate::{assembler::Assembler, vm::VM};

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

    pub fn run_program(&mut self, program: Vec<&str>) {
        for command in &program {
            info!("Running command: {}", command);
            if self.is_directive(command) {
                self.run_directive(command);
            } else {
                self.execute_command(command);
            }
        }
    }

    pub fn execute_command(&mut self, command: &str) {
        let parsed_program = Assembler::parse_instruction(command);

        match parsed_program {
            Ok(instruction) => {
                let bytes_command = instruction.to_bytes();

                // Dont need this part. For test
                let hex_command = instruction.to_hex();
                debug!("Hex: {}", hex_command.join(" "));
                //

                debug!("Bytes command {:?}", bytes_command);
                for byte in bytes_command {
                    self.vm.add_byte(byte)
                }
                self.vm.run_once();
            }
            Err(_) => {
                error!("Unable to decode command string")
            }
        }
    }

    pub fn run_hex_program(&mut self, hex_program: Vec<&str>) {
        hex_program.iter().for_each(|command| {
            self.execute_hex_command(command);
        });
    }

    /* or directive */
    pub fn execute_hex_command(&mut self, command: &str) {
        self.commands.push(command.to_string());
        if self.is_directive(command) {
            self.run_directive(command);
        } else {
            let results = self.parse_hex(command);
            match results {
                Ok(bytes) => {
                    for byte in bytes {
                        self.vm.add_byte(byte)
                    }
                }
                Err(_e) => {
                    error!(
                        "Unable to decode hex string. Please enter 4 groups of 2 hex characters."
                    )
                }
            };
            self.vm.run_once();
        }
    }

    fn is_directive(&self, command: &str) -> bool {
        command.starts_with('.')
    }

    fn run_directive(&mut self, command: &str) {
        match command {
            ".program" => {
                info!("Listing instructions currently in VM's program vector:");
                for instruction in &self.vm.program {
                    info!("{}", instruction);
                }
                info!("End of Program Listing");
            }
            ".commands" => {
                for command in &self.commands {
                    info!("{}", command);
                }
            }
            ".registers" => {
                info!("Listing registers and all contents:");
                info!("{:#?}", self.vm.registers);
                info!("End of Register Listing")
            }
            ".equal_flag" => {
                info!("Equal flag: {}", self.vm.equal_flag);
            }
            ".quit" => {
                info!("Farewell! Have a great day!");
                std::process::exit(0);
            }
            _ => {}
        }
    }

    /// Accepts a hexadecimal string WITHOUT a leading `0x` and returns a Vec of u8
    /// Example for a LOAD command: 00 01 03 E8
    fn parse_hex(&self, input: &str) -> Result<Vec<u8>, ParseIntError> {
        let mut results: Vec<u8> = vec![];

        let byte_results = input
            .split(' ')
            .map(|hex_str| u8::from_str_radix(hex_str, 16));

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
