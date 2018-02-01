use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
mod vm;
mod asm;

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
        vm::initialize();
        vm::copy_program(asm::assemble(lines_from_file("/home/taco/Documents/rust/asm/src/prog.asm").iter().map(AsRef::as_ref).collect()));
        vm::run();
    }
}
