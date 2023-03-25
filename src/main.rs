#![deny(clippy::all)]

mod instruction;
mod lexer;
mod repl;
mod vm;

use repl::Repl;
use std::io::{self, Write};

use log::LevelFilter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .format_timestamp(None)
        .init();

    let mut repl = Repl::new();

    main_program(&mut repl)
    // main_hex_program(&mut repl)
    // main_hex_lopp(&mut repl)
}

#[allow(dead_code)]
fn main_hex_lopp(repl: &mut Repl) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let stdin = io::stdin();
        print!(">>> ");
        io::stdout().flush().expect("Unable to flush stdout");
        let mut buffer = String::new();
        stdin
            .read_line(&mut buffer)
            .expect("Unable to read line from user");
        let buffer = buffer.trim();

        repl.execute_hex_command(buffer);
    }
}

#[allow(dead_code)]
fn main_hex_program(repl: &mut Repl) -> Result<(), Box<dyn std::error::Error>> {
    // HEX program
    #[rustfmt::skip]
    let hex_program: Vec<&str> = vec![
        ".registers",
        ".program",
        "00 01 03 E8",   // LOAD $#1 #((0000.0011 << 8) + 1110.1000 = 1000)
        ".registers",
        ".commands",
    ];

    repl.run_hex_program(hex_program);

    Ok(())
}

fn main_program(repl: &mut Repl) -> Result<(), Box<dyn std::error::Error>> {
    // program
    #[rustfmt::skip]
    let program: Vec<&str> = vec![
        ".registers",
        ".program",
        "load $1 #1000",   // LOAD $#1 #((0000.0011 << 8) + 1110.1000 = 1000)
        "load $0 #999",
            
        "add $0 $1 $2",
        "sub $1 $0 $1",
        
        "load $3 #4",
        "jmpf $3",         // TODO: Need to fix jump. Now it jump $3 bytes. 
        "sub $2 $0 $2",

        "load $4 #4",
        "eq $3 $4 $4",
        ".equal_flag",
        
        ".registers",
        ".commands",
    ];

    repl.run_program(program);
    Ok(())
}

// https://www.rapidtables.com/convert/number/hex-to-decimal.html
// https://www.rapidtables.com/convert/number/binary-to-decimal.html

/*
0b 0000.0000.0000.0000 = 2^4 * 2^4 * 2^4 * 2*4 = 2^16
0h (0-9-A-F) = 16 = 2^4 = 4bits
   (0-9-A-F)(0-9-A-F) = 16*16 = 2^4*2^4 = 2^8 = 8bits
*/
