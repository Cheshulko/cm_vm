mod instruction;
mod repl;
mod vm;

use repl::Repl;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut repl = Repl::new();

    // HEX program
    #[rustfmt::skip]
    let program: Vec<&str> = vec![
        ".registers",
        ".program",
        "00 01 03 E8",   // LOAD $#1 #((0000.0011 << 8) + 1110.1000 = 1000)
        ".registers",
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
