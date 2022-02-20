use crate::brainfuck::operator::Operator;
use std::io::Read;

pub struct VM<const MEM_SIZE: usize> {
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
                    self.memory_pointer = self.memory_pointer.wrapping_sub(1) % MEM_SIZE;
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
            _ => {}
        }

        ip += 1;
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
            _ => {}
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
