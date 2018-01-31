mod vm;
mod asm;

fn main() {
    unsafe{
        vm::initialize();
        let test_program =
        vec![
            asm::assemble("inc 16"),
            asm::assemble("add $16 16"),
            asm::assemble("eof")
        ];
        vm::copy_program(test_program);
        vm::run();
    }
}
