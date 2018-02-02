extern crate hex;
extern crate minifb;
extern crate time;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::env;
use std::time::Duration;
mod vm;
mod asm;
mod gfx;

use minifb::{Key, Scale, WindowOptions, Window};

const WIDTH: usize = 210;
const HEIGHT: usize = 210;
//todo: implement macros at the assembler level
//implement memory paging and virtual addresses
//implement registers
//implement stacks ESPECIALLY for strings
//initialize constant addresses for every english unicode letter, and some symbols.

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
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Test - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions{
                                     scale: Scale::X4,
                                     ..WindowOptions::default()
                                 }).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    unsafe {
        let args: Vec<_> = env::args().collect();
        if args.len() < 2 { panic!("Missing file argument"); }
        vm::initialize();
        vm::copy_program(asm::assemble(lines_from_file(&args[1]).iter().map(AsRef::as_ref).collect()));
    }

    let mut curPix = 0;
    let mut frame = 0;
    unsafe{ while window.is_open() && !window.is_key_down(Key::Escape) && vm::RAM[4094] != 1{

            vm::run();
            if vm::RAM[5] != 0 {
                buffer[curPix] = vm::RAM[5];
                curPix = curPix+1;
                if curPix >= buffer.len() {
                    window.update_with_buffer(&buffer).unwrap();
                    curPix = 0;
                }
                vm::RAM[5] = 0;
            }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        //window.update_with_buffer(&buffer).unwrap();
        frame=frame+1;
    }
    }
    println!();
}
