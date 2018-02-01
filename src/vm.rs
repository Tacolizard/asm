extern crate time;
use asm;

//16bit memory space, 8bit opcode,  12 bit addresses
//first two hex digits are opcode, next 6 are two 3 digit addresses
pub static mut RAM: [u32; 4095] = [0xDEADBEEF; 4095];
const DPRINT: bool = true;
pub const SYSTEM_OFFSET: u32 = 2;//index of where the program should start being mapped into ram

macro_rules! dprintln {
    ($expression:expr) => (
        if DPRINT == true {
            println!($expression);
        }
    );
    ($expression:expr, $($arg:tt)*) => (
        if DPRINT == true {
            println!($expression, $($arg)*);
        }
    );
}

pub unsafe fn initialize() {
    RAM[0] = SYSTEM_OFFSET;
    RAM[1] = 0xC001BABE;
}

pub unsafe fn copy_program(prog: Vec<u32>) {
    let mut i = 0;
    for n in prog {
        RAM[i+SYSTEM_OFFSET as usize] = n;
        i=i+1;
    }
}

pub unsafe fn step() {
    let eip = RAM[0];
    let cache_eip = eip;
    exec(RAM[eip as usize], false);
    if RAM[0] == cache_eip {RAM[0] = RAM[0] + 0x1};//if an instruction has modified the eip, don't overwrite the change
}

pub unsafe fn run() {//0x0FFE
    while RAM[4094] != 1 {
        let start = time::precise_time_s();
        step();
        while time::precise_time_s() - start < 0.0001 {}
    }
}

pub unsafe fn exec(space: u32, silent: bool) {
    //at this level there are no numerical constants, the assembler will
    //assign the constant value to some open address and all references to the
    //constant will point to that address
    let opcode = (space).rotate_left(8) as u8; //get the first two digits of space
    let arg1 = (0x00_FFF_000 & space) >> 12;
    let arg2 = 0x00_000_FFF & space;
    if !silent { dprintln!("{}: OPCODE: {:X}\nARG1: {:X}, ARG2: {:X}\nBEFORE: {:X}", RAM[0], opcode, arg1, arg2, RAM[arg2 as usize]); }

    if opcode == 0xFF { //break execution
        RAM[4094] = 0x01;
    }
    if opcode == 0x01 { //inc
        RAM[arg1 as usize] = RAM[arg1 as usize] + 0x01;
    }
    if opcode == 0x02 { //dec
        RAM[arg1 as usize] = RAM[arg1 as usize] - 0x01;
    }
    if opcode == 0x03 { //add
        RAM[arg2 as usize] = RAM[arg2 as usize] + RAM[arg1 as usize];
    }
    if opcode == 0x04 { //sub
        RAM[arg2 as usize] = RAM[arg2 as usize] - RAM[arg1 as usize];
    }
    if opcode == 0x05 { //mov
        exec_str_vec(vec![
                        &format!("sub {} {}",arg2, arg2),
                        &format!("add {} {}",arg1, arg2)
                    ]);
    }
    if opcode == 0x06 { //jmp
        RAM[0] = arg1;
    }
    if opcode == 0x07 { //cmp
        let r = RAM[arg2 as usize] - RAM[arg1 as usize];
        exec_str(&format!("mov {} 1", r));
    }
    if !silent { dprintln!("AFTER: {:X}\n", RAM[arg2 as usize]); }
}

pub unsafe fn exec_str(s: &str) {//execute unassembled string
    exec(asm::translate(s), true);
}

pub unsafe fn exec_str_vec(p: Vec<&str>) {//execute unassembled vec
    exec_vec(asm::assemble(p));
}

pub unsafe fn exec_vec(pro: Vec<u32>) {//execute an assembled vec
    for n in pro {
        exec(n, true);
    }
}
