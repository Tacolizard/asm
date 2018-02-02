extern crate hex;
extern crate minifb;
extern crate time;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::env;
mod vm;
mod asm;
mod gfx;

use minifb::{Key, Scale, WindowOptions, Window};

const WIDTH: usize = 210;
const HEIGHT: usize = 210;
//pub static mut BUFFER: Vec<u32> = vec![0; WIDTH * HEIGHT];
pub static mut BUFFER: [u32; WIDTH*HEIGHT] = [0; WIDTH * HEIGHT];
pub static mut CURPIX: u32 = 0;

//todo:
//implement some way for a program to display and move sprites using gfx
//implement macros at the assembler level
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

pub unsafe fn push_buffer(win: &mut Window, color: &mut u32) {
    if *color != 0 {
        BUFFER[CURPIX as usize] = *color;
        CURPIX = CURPIX + 1;
        if CURPIX as usize >= BUFFER.len() {
            CURPIX = 0;
            //win.update_with_buffer(&BUFFER).unwrap();
        }
        *color = 0;
    }
}

fn main() {
    //let mut BUFFER: Vec<u32> = vec![0; WIDTH * HEIGHT];

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
        gfx::initialize();
    }

    let mut frame = 0;
    unsafe{
        while window.is_open() && !window.is_key_down(Key::Escape) && vm::RAM[4094] != 1{

            vm::run();
            push_buffer(&mut window, &mut vm::RAM[5]);

            if frame % 9999 == 0 {
                let mut bg = BUFFER; //swap buffers
                BUFFER = gfx::update(BUFFER, frame);
                window.update_with_buffer(&BUFFER).unwrap();
                BUFFER = bg;
            }
            frame=frame+1;
        }
    }
    println!();
}
