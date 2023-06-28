# cm_vm

## Overview
Virtual machine for the purpose of primitive studies

1) Can run commands program, ex. `Repl.run_program(program)`. Each command converts to bytes command `Assembler::parse_instruction() -> AssemblerInstruction`

Program example:
```
",registers",
",program",
"load $1 #1000",   // LOAD $#1 #((0000.0011 << 8) + 1110.1000 = 1000)
"load $0 #999",
"add $0 $1 $2",
"sub $1 $0 $1",
"load $3 #4",
"jmpf $3", 
"sub $2 $0 $2",
"load $4 #4",
"eq $3 $4 $4",
",equal_flag",
".uselessdirective $0 $1 #100",
",registers",
",program",

```

2) Can run Assembler hex instuctions, ex. `Repl.run_hex_program(hex_program)`

Program example: 
```
",registers",
",program",
"00 01 03 E8",   // LOAD $#1 #((0000.0011 << 8) + 1110.1000 = 1000)
",registers",
",commands",
",program",
```

Supported directives: 
```
,program - Listing instructions currently in VM's program vector
,registers - Listing registers and all contents:
,equal_flag - VM equel flag state (true/false)
,quit - Quit 
```

Supported tokens:
```
Op { code: Opcode },
Register { reg_num: u8 },
Number { value: i32 },
Directive { name: String },
```
Supported instuctions: 
```
LOAD, // `LOAD $0 #500` Load value into register $0. #500 16 bits
ADD,  // `ADD $0 $1 $2` add $0 + $1 registers. Save to $2 register
SUB,  // `SUB $0 $1 $2` sub $0 - $1 registers. Save to $2 register
MUL,  // `MUL $0 $1 $2` mul $0 * $1 registers. Save to $2 register
DIV,  // `DIV $0 $1 $2` mul $0 / $1 registers. Save to $2 register. Use `remainder`
HLT,  // Stop execution
JMP,  // `JMP $0`.  Jump to $0 program byte.  Absolute jump 
JMPF, // `JMPF $0`  Jump forwards by $0.      Relative jump
JMPB, // `JMPB $0`  Jump backwards by $0.     Relative jump  
EQ,   // `EQ  $0 $1 $unused`  aka ($0 == $1) Save result to `equal_flag`. Equal
NEQ,  // `NEQ $0 $1 $unused`  aka ($0 != $1) Save result to `equal_flag`. Not equal
GT,   // `GT  $0 $1 $unused`  aka ($0 >  $1) Save result to `equal_flag`. Greater than
LT,   // `LT  $0 $1 $unused`  aka ($0 <  $1) Save result to `equal_flag`. Less than
GTQ,  // `GTQ $0 $1 $unused`  aka ($0 >= $1) Save result to `equal_flag`. Greater than OR equal to
JEQ,  // `JEQ $0` Jump to $0 if equal (`equal_flag` is true)  Absolute jump 
ALOC, // `ALOC $0` Allocate $0 bytes of memory in the heap
IGL,  // Illegal
```
