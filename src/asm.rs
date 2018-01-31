use vm;
use std::collections::HashMap;

pub unsafe fn assemble(prog: Vec<&str>) -> Vec<u32> {
    let mut constants = HashMap::new();
    for ln in prog {
        
    }
}

pub unsafe fn translate(inst: &str) -> u32{
    //this function is the lowest-level assembling function.
    //most other parts of assembly will be handled via preprocessing
    //the entire program. This will require some refactors however.

    //this function works by creating a hex with all zeroes and then
    //using bitwise operators to 'mask in' and 'concatenate' the appropriate
    //values.
    let mut out_inst = 0x00_000_000;
    let pieces = inst.split(" ").collect::<Vec<&str>>();
    let string_opcode = pieces[0];
    let mut string_arg1 = String::from("0");
    let mut string_arg2 = String::from("0");
    if pieces.len() >= 2 {
        string_arg1 = String::from(pieces[1]);
    }
    if pieces.len() == 3 {
        string_arg2 = String::from(pieces[2]);
    }
    /*if string_arg1.starts_with("$") { //constants
        let mut i=0;    //currently constants with the same value will not use shared addresses.
        for n in vm::RAM.iter() { //in the future constants will be handled by preprocessing.
            if n == &0xDEADBEEF && i > 3000{
                let num = (string_arg1.trim_left_matches("$")).parse::<u32>().expect("Arg1 constant error.");
                vm::RAM[i] = num;
                break;
            }
            i=i+1;
        }
        string_arg1 = i.to_string();
    }
    if string_arg2.starts_with("$") {
        let mut i=0;
        for n in vm::RAM.iter() {
            if n == &0xDEADBEEF && i > 3000{
                let num = (string_arg2.trim_left_matches("$")).parse::<u32>().expect("Arg2 constant error.");
                vm::RAM[i] = num;
                break;
            }
            i=i+1;
        }
        string_arg2 = i.to_string();
    }*/

    //mask in first argument
    out_inst = (0x00_FFF_000 & (string_arg1.parse::<u32>().expect("Arg1 masking error.") << 12)) | out_inst;
    //mask in second argument
    out_inst = (0x00_000_FFF & (string_arg2.parse::<u32>().expect("Arg2 masking error."))) | out_inst;

    if string_opcode == "eof" {
        out_inst = 0xFF_000_000 | out_inst; //apply opcode mask using bitwise OR
    }
    if string_opcode == "inc" {
        out_inst = 0x01_000_000 | out_inst;
    }
    if string_opcode == "dec" {
        out_inst = 0x02_000_000 | out_inst;
    }
    if string_opcode == "add" {
        out_inst = 0x03_000_000 | out_inst;
    }
    if string_opcode == "sub" {
        out_inst = 0x04_000_000 | out_inst;
    }
    if string_opcode == "jmp" {
        out_inst = 0x05_000_000 | out_inst;
    }

    return out_inst;
}
