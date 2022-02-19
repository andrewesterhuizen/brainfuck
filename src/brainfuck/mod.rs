pub mod operator;
mod parser;
mod vm;

use parser::Parser;
use vm::VM;

pub struct Brainfuck {
    parser: Parser,
    vm: VM<32768>,
}

impl Brainfuck {
    pub fn new() -> Brainfuck {
        Brainfuck {
            parser: Parser::new(),
            vm: VM::new(),
        }
    }

    pub fn run(&mut self, source: String) {
        let instructions = self.parser.run(source);
        self.vm.run(&instructions);
    }
}
