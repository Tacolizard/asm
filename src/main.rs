mod vm;
mod asm;

fn main() {
    unsafe{
        vm::initialize();
        let test_program =
        vec![
            "sub 16 16",
            "add $3735928559 16",
            "sub $0xDEADBEEF 16",
            "eof"
        ];
        vm::copy_program(asm::assemble(test_program));
        vm::run();
    }
}
