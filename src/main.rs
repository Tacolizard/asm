mod vm;
mod asm;

fn main() {
    unsafe{
        vm::initialize();
        let test_program =
        vec![
            "sub 16 16",
            "sub 17 17",
            "add $3735928559 16",
            "mov $0xDEAD 17",
            "eof"
        ];
        vm::copy_program(asm::assemble(test_program));
        vm::run();
    }
}
