extern crate time;
extern crate hex;
use std::io;
use asm;

//16bit memory space, 8bit opcode,  12 bit addresses
//first two hex digits are opcode, next 6 are two 3 digit addresses
pub static mut RAM: [u32; 4095] = [0xDEADBEEF; 4095];
const DPRINT: bool = false;
pub const SYSTEM_OFFSET: u32 = 5;//index of where the program should start being mapped into ram

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
    RAM[0] = SYSTEM_OFFSET; //ip
    RAM[1] = 0xC001BABE; //ret
    RAM[2] = 0x00000000; //EFLAGS
    RAM[3] = 0x00000000; //stdout
    RAM[4] = 0x00000000; //stdin
}

pub unsafe fn copy_program(prog: Vec<u32>) {
    let mut i = 0;
    for n in prog {
        RAM[i+SYSTEM_OFFSET as usize] = n;
        i=i+1;
    }
}

pub unsafe fn step() {
    let ip = RAM[0];
    let cache_ip = ip;
    exec(RAM[ip as usize], false);
    if RAM[0] == cache_ip {RAM[0] = RAM[0] + 0x1};//if an instruction has modified the ip, don't overwrite the change
}

pub unsafe fn run() {//0x0FFE
    while RAM[4094] != 1 {
        let start = time::precise_time_s();
        let stdin = io::stdin();
        let input = &mut String::new();
        step();
        if (RAM[2] as u32 & (1u32<<12))>>12 == 1 {
            input.clear();
            stdin.read_line(input);
            RAM[4] = u32::from_str_radix(&hex::encode(input.trim_matches('\n')), 16).unwrap();
            RAM[2] = RAM[2] ^ 0x0_0_0_0_1_0_0_0;
        }

        if RAM[3] != 0 {
            let tv: Vec<u8> = format!("{:X}", RAM[3]).into_bytes();
            println!(" >> {}", String::from_utf8(hex::decode(tv).unwrap()).unwrap());
            RAM[3] = 0;
        }
        while time::precise_time_s() - start < 1.0 {}
    }
}

pub unsafe fn exec(space: u32, silent: bool) {
    //at this level there are no numerical constants, the assembler will
    //assign the constant value to some open address and all references to the
    //constant will point to that address
    let opcode = (space).rotate_left(8) as u8; //get the first two digits of space
    let arg1 = (0x00_FFF_000 & space) >> 12;
    let arg2 = 0x00_000_FFF & space;
    if !silent { dprintln!("{}: OPCODE: {:X}\nARG1: {:X}, ARG2: {:X}\nBEFORE: {:X}", RAM[0], opcode, arg1, arg2, RAM[arg1 as usize]); }
    let zf: u32 = (RAM[2] & (1u32<<0))>>0;
    let of: u32 = (RAM[2] & (1u32<<4))>>4;
    let sf: u32 = (RAM[2] & (1u32<<8))>>8;


    if opcode == 0xFF { //break execution
        RAM[4094] = 0x01;
    }
    if opcode == 0x01 { //inc
        //RAM[arg1 as usize] = RAM[arg1 as usize] + 0x01;
        let r = RAM[arg1 as usize].checked_add(0x01);
        match r {
            Some(t) => {
                RAM[1] = t;
                RAM[arg1 as usize] = t;
                if t == 0 {
                    RAM[2] = 0x0_0_0_0_0_0_0_1;
                }
            },
            None => {
                dprintln!("Overflow.");
                let mut flags = 0x0_0_0_0_0_0_0_1_0;
                if RAM[arg1 as usize] < RAM[arg2 as usize] { flags = 0x0_0_0_0_0_0_1_0_0 | flags; }
                RAM[2] = flags;
            },
        };
    }
    if opcode == 0x02 { //dec
        //RAM[arg1 as usize] = RAM[arg1 as usize] - 0x01;
        let r = RAM[arg1 as usize].checked_sub(0x01);
        match r {
            Some(t) => {
                RAM[1] = t;
                RAM[arg1 as usize] = t;
                if t == 0 {
                    RAM[2] = 0x0_0_0_0_0_0_0_1;
                }
            },
            None => {
                dprintln!("Overflow.");
                let mut flags = 0x0_0_0_0_0_0_0_1_0;
                if RAM[arg1 as usize] < RAM[arg2 as usize] { flags = 0x0_0_0_0_0_0_1_0_0 | flags; }
                RAM[2] = flags;
            },
        };
    }
    if opcode == 0x03 { //add
        //RAM[arg1 as usize] = RAM[arg1 as usize] + RAM[arg2 as usize];
        let r = RAM[arg1 as usize].checked_add(RAM[arg2 as usize]);
        match r {
            Some(t) => {
                RAM[1] = t;
                RAM[arg1 as usize] = t;
                if t == 0 {
                    RAM[2] = 0x0_0_0_0_0_0_0_1;
                }
            },
            None => {
                dprintln!("Overflow.");
                let mut flags = 0x0_0_0_0_0_0_0_1_0;
                if RAM[arg1 as usize] < RAM[arg2 as usize] { flags = 0x0_0_0_0_0_0_1_0_0 | flags; }
                RAM[2] = flags;
            },
        };
    }
    if opcode == 0x04 { //sub
        //RAM[arg1 as usize] = RAM[arg1 as usize] - RAM[arg2 as usize];
        let r = RAM[arg1 as usize].checked_sub(RAM[arg2 as usize]);
        match r {
            Some(t) => {
                RAM[1] = t;
                RAM[arg1 as usize] = t;
                if t == 0 {
                    RAM[2] = 0x0_0_0_0_0_0_0_1;
                }
            },
            None => {
                dprintln!("Overflow.");
                let mut flags = 0x0_0_0_0_0_0_0_1_0;
                if RAM[arg1 as usize] < RAM[arg2 as usize] { flags = 0x0_0_0_0_0_0_1_0_0 | flags; }
                RAM[2] = flags;
            },
        };
    }
    if opcode == 0x05 { //mov
        RAM[arg1 as usize] = RAM[arg2 as usize];
        //i've decided not to macro this because real CPUs can directly copy
        //values from and into addresses.
    }
    if opcode == 0x06 { //jmp
        RAM[0] = arg1;
    }
    if opcode == 0x07 { //cmp
        let r = RAM[arg1 as usize].checked_sub(RAM[arg2 as usize]);
        match r {
            Some(t) => {
                RAM[1] = t;
                if t == 0 {
                    RAM[2] = 0x0_0_0_0_0_0_0_1;
                }
            },
            None => {
                dprintln!("Overflow.");
                let mut flags = 0x0_0_0_0_0_0_0_1_0;
                if RAM[arg1 as usize] < RAM[arg2 as usize] { flags = 0x0_0_0_0_0_0_1_0_0 | flags; }
                RAM[2] = flags;
            },
        };
    }
    if opcode == 0x08 { //je
        if zf == 1 {
            RAM[0] = arg1;
        }
    }
    if opcode == 0x09 { //jne
        if zf == 0 {
            RAM[0] = arg1;
        }
    }
    if opcode == 0x0A { //ja
        if of == zf && sf == 0 {
            RAM[0] = arg1;
        }
    }
    if opcode == 0x0B { //jae
        if sf == 0 || zf == 1 {
            RAM[0] = arg1;
        }
    }
    if opcode == 0x0C { //jo, jump if overflow
        if of == 1 {
            RAM[0] = arg1;
        }
    }
    if opcode == 0x0D { //jno
        if of == 0 {
            RAM[0] = arg1;
        }
    }
    if opcode == 0x0F { //js, jump if signed
        if sf == 1 {
            RAM[0] = arg1;
        }
    }
    if opcode == 0x10 { //jns
        if sf == 0 {
            RAM[0] = arg1;
        }
    }
    if opcode == 0x11 { //and
        RAM[arg1 as usize] = RAM[arg1 as usize] & RAM[arg2 as usize];
    }
    if opcode == 0x12 { //or
        RAM[arg1 as usize] = RAM[arg1 as usize] | RAM[arg2 as usize];
    }
    if opcode == 0x13 { //xor
        RAM[arg1 as usize] = RAM[arg1 as usize] ^ RAM[arg2 as usize];
    }

    if !silent { dprintln!("zf:{} of:{} sf:{}",(RAM[2] & (1u32<<0))>>0,(RAM[2] & (1u32<<4))>>4,(RAM[2] & (1u32<<8))>>8); }
    if !silent { dprintln!("AFTER: {:X}\nRET: {:X}\n", RAM[arg1 as usize], RAM[1]); }
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
