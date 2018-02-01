use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::env;
mod vm;
mod asm;

//todo: implement macros at the assembler level
//implement memory paging and virtual addresses so i can have stacks and stuff
//implement registers

fn lines_from_file<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    return buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    unsafe{
        let args: Vec<_> = env::args().collect();
        if args.len() < 2 { panic!("Missing file argument"); }
        vm::initialize();
        vm::copy_program(asm::assemble(lines_from_file(&args[1]).iter().map(AsRef::as_ref).collect()));
        vm::run();
    }
}
