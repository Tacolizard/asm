mod vm;
mod asm;

fn main() {
    unsafe{
        vm::initialize();
        let test_program =
        vec![
            "inc 16",
            "add $16 16",
            "eof"
        ];
        vm::copy_program(asm::assemble(test_program));
        vm::run();
    }
}
