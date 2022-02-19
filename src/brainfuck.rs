use std::io::Read;

#[repr(u8)]
#[derive(Debug, PartialEq)]
enum Operator {
    IncrementPointer, // >  -  move the pointer right
    DecrementPointer, // <  -  move the pointer left
    IncrementCell,    // +  -  increment the current cell
    DecrementCell,    // -  -  decrement the current cell
    Output,           // .  -  output the value of the current cell
    Input,            // ,  -  replace the value of the current cell with input
    LoopStart,        // [  -  jump to the matching ] instruction if the current value is zero
    LoopEnd,          // ]  -  jump to the matching [ instruction if the current value is not zero
}

struct VM<const MEM_SIZE: usize> {
    memory_pointer: usize,
    instruction_pointer: usize,
    memory: [u8; MEM_SIZE],
}

impl<const MEM_SIZE: usize> VM<MEM_SIZE> {
    pub fn new() -> VM<MEM_SIZE> {
        VM {
            memory_pointer: 0,
            instruction_pointer: 0,
            memory: [0; MEM_SIZE],
        }
    }

    pub fn run(&mut self, program: &Vec<Operator>) {
        while self.instruction_pointer < program.len() {
            let op = &program[self.instruction_pointer];

            match op {
                Operator::DecrementPointer => {
                    self.memory_pointer = self.memory_pointer.wrapping_sub(1) % MEM_SIZE
                }
                Operator::IncrementPointer => {
                    self.memory_pointer = self.memory_pointer.wrapping_add(1) % MEM_SIZE;
                }
                Operator::DecrementCell => {
                    self.memory[self.memory_pointer] =
                        self.memory[self.memory_pointer].wrapping_sub(1);
                }
                Operator::IncrementCell => {
                    self.memory[self.memory_pointer] =
                        self.memory[self.memory_pointer].wrapping_add(1);
                }
                Operator::Output => {
                    print!("{}", self.memory[self.memory_pointer] as char);
                }
                Operator::Input => {
                    self.memory[self.memory_pointer] = read_char();
                }
                Operator::LoopStart => {
                    if self.memory[self.memory_pointer] == 0 {
                        self.instruction_pointer =
                            find_next_loop_end_address(self.instruction_pointer, program);
                    }
                }
                Operator::LoopEnd => {
                    if self.memory[self.memory_pointer] != 0 {
                        self.instruction_pointer =
                            find_previous_loop_start_address(self.instruction_pointer, program);
                    }
                }
            }

            self.instruction_pointer = self.instruction_pointer.wrapping_add(1);
        }
    }
}

fn find_next_loop_end_address(loop_start_address: usize, program: &Vec<Operator>) -> usize {
    assert!(loop_start_address < program.len());

    let mut ip = loop_start_address + 1;
    let mut nest_level = 0;

    while ip < program.len() {
        let op = &program[ip];

        match op {
            Operator::LoopEnd => {
                if nest_level == 0 {
                    return ip;
                }
                nest_level -= 1;
            }
            Operator::LoopStart => {
                nest_level += 1;
            }
            _ => (),
        }

        ip += 1;
        println!();
    }

    assert!(false, "no matching ] found");
    return 0;
}

fn find_previous_loop_start_address(loop_end_address: usize, program: &Vec<Operator>) -> usize {
    assert!(loop_end_address > 0);

    let mut ip = loop_end_address - 1;
    let mut nest_level = 0;

    while ip < program.len() {
        let op = &program[ip];

        match op {
            Operator::LoopEnd => {
                nest_level += 1;
            }
            Operator::LoopStart => {
                if nest_level == 0 {
                    return ip;
                }
                nest_level -= 1;
            }
            _ => (),
        }

        ip -= 1;
    }

    assert!(false, "no matching [ found");
    return 0;
}

#[test]
fn find_next_loop_end_address_finds_address_no_nesting() {
    let program = vec![Operator::LoopStart, Operator::LoopEnd];
    let addr = find_next_loop_end_address(0, &program);
    assert_eq!(addr, 1);
}

#[test]
fn find_next_loop_end_address_finds_address_nested() {
    let program = vec![
        Operator::LoopStart,
        Operator::LoopStart,
        Operator::LoopEnd,
        Operator::LoopEnd,
    ];
    let addr = find_next_loop_end_address(0, &program);
    assert_eq!(addr, 3);
}

#[test]
fn find_next_loop_end_address_finds_address_nested_starting_in_inner_loop() {
    let program = vec![
        Operator::LoopStart,
        Operator::LoopStart,
        Operator::LoopEnd,
        Operator::LoopEnd,
    ];
    let addr = find_next_loop_end_address(1, &program);
    assert_eq!(addr, 2);
}

#[test]
fn find_previous_loop_start_address_finds_address_no_nesting() {
    let program = vec![Operator::LoopStart, Operator::LoopEnd];
    let addr = find_previous_loop_start_address(1, &program);
    assert_eq!(addr, 0);
}

#[test]
fn find_previous_loop_start_address_finds_address_nested() {
    let program = vec![
        Operator::LoopStart,
        Operator::LoopStart,
        Operator::LoopEnd,
        Operator::LoopEnd,
    ];
    let addr = find_previous_loop_start_address(3, &program);
    assert_eq!(addr, 0);
}

#[test]
fn find_previous_loop_start_address_finds_address_nested_starting_in_inner_loop() {
    let program = vec![
        Operator::LoopStart,
        Operator::LoopStart,
        Operator::LoopEnd,
        Operator::LoopEnd,
    ];
    let addr = find_previous_loop_start_address(2, &program);
    assert_eq!(addr, 1);
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
                '<' => Operator::DecrementPointer,
                '>' => Operator::IncrementPointer,
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
        Operator::IncrementPointer,
        Operator::DecrementPointer,
        Operator::IncrementCell,
        Operator::DecrementCell,
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
