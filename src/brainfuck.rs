use std::io::Read;

#[repr(u8)]
#[derive(Debug, PartialEq)]
enum Operator {
    DecrementPointer, // >  -  move the pointer right
    IncrementPointer, // <  -  move the pointer left
    DecrementCell,    // +  -  increment the current cell
    IncrementCell,    // -  -  decrement the current cell
    Output,           // .  -  output the value of the current cell
    Input,            // ,  -  replace the value of the current cell with input
    LoopStart,        // [  -  jump to the matching ] instruction if the current value is zero
    LoopEnd,          // ]  -  jump to the matching [ instruction if the current value is not zero
}

struct VM {
    memory_pointer: usize,
    instruction_pointer: usize,
    memory: [u8; 256],
}

impl VM {
    pub fn new() -> VM {
        VM {
            memory_pointer: 0,
            instruction_pointer: 0,
            memory: [0; 256],
        }
    }

    pub fn run(&mut self, program: &Vec<Operator>) {
        while self.instruction_pointer < program.len() {
            let op = &program[self.instruction_pointer];

            match op {
                Operator::DecrementPointer => {
                    self.memory_pointer -= 1;
                    self.instruction_pointer += 1;
                }
                Operator::IncrementPointer => {
                    self.memory_pointer += 1;
                    self.instruction_pointer += 1;
                }
                Operator::DecrementCell => {
                    self.memory[self.memory_pointer] -= 1;
                    self.instruction_pointer += 1;
                }
                Operator::IncrementCell => {
                    self.memory[self.memory_pointer] += 1;
                    self.instruction_pointer += 1;
                }
                Operator::Output => {
                    print!("{}", self.memory[self.memory_pointer] as char);
                    self.instruction_pointer += 1;
                }
                Operator::Input => {
                    self.memory[self.memory_pointer] = read_char();
                    self.instruction_pointer += 1;
                }
                Operator::LoopStart => todo!(),
                Operator::LoopEnd => todo!(),
            }
        }
    }
}

fn read_char() -> u8 {
    std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .unwrap()
}

struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn run(&self, source: String) -> Vec<Operator> {
        let mut program = Vec::new();

        for c in source.chars() {
            let op = match c {
                '>' => Operator::DecrementPointer,
                '<' => Operator::IncrementPointer,
                '-' => Operator::DecrementCell,
                '+' => Operator::IncrementCell,
                '.' => Operator::Output,
                ',' => Operator::Input,
                '[' => Operator::LoopStart,
                ']' => Operator::LoopEnd,
                _ => continue,
            };

            program.push(op);
        }

        program
    }
}

#[test]
fn parser_works() {
    let program = "><+-.,[]".to_string();
    let instructions = Parser::new().run(program);

    let expected = vec![
        Operator::DecrementPointer,
        Operator::IncrementPointer,
        Operator::DecrementCell,
        Operator::IncrementCell,
        Operator::Output,
        Operator::Input,
        Operator::LoopStart,
        Operator::LoopEnd,
    ];

    assert_eq!(instructions, expected);
}

#[test]
fn parser_ignores_unknown_characters() {
    let program = "1234567890qwertyuiopasdfghjklzxcvbnm\n\r\t".to_string();
    let instructions = Parser::new().run(program);
    assert_eq!(instructions.len(), 0);
}

pub struct Brainfuck {
    parser: Parser,
    vm: VM,
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
