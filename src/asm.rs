use vm;
use std::collections::HashMap;

pub fn multiparse(top: &str) -> u32{
    if top.starts_with("0x") {
        return u32::from_str_radix(top.trim_left_matches("0x"), 16).expect("hex conversion err");
    }

    return top.parse::<u32>().expect("Const err.");
}

pub unsafe fn bind_open(val: u32) -> u32 {
    let mut i=3095;//find an open address and assign a value to it
    for l in vm::RAM.iter() {
        if l == &0xDEADBEEFu32 {
            vm::RAM[i as usize] = val;
            return i as u32;
        }
        i=i+1;
    }
    return 4093;
}

pub unsafe fn assemble(prog: Vec<&str>) -> Vec<u32> {
    let mut constants = HashMap::new();
    let mut prog_preprocessed = Vec::new();
    let mut prog_translated = Vec::new();

    for ln in prog { //resolve constants
        let mut outln = String::from(ln);
        for piece in ln.split(" ") {
            if piece.chars().next().unwrap() == '$' {
                if !constants.contains_key(piece) {
                    constants.insert(piece, bind_open(multiparse(piece.trim_left_matches("$"))));
                }
                outln = outln.replace(piece, &constants.get(piece).unwrap().to_string());
            }
        }
        prog_preprocessed.push(outln);
    }
    for ln in prog_preprocessed {
        println!("{}", ln);
        prog_translated.push(translate(&ln));
    }

    return prog_translated;

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
    if string_opcode == "mov" {
        out_inst = 0x05_000_000 | out_inst;
    }
    if string_opcode == "jmp" {
        out_inst = 0x06_000_000 | out_inst;
    }
    if string_opcode == "cmp" {
        out_inst = 0x07_000_000 | out_inst;
    }

    return out_inst;
}
