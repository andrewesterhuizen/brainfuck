use std::io::Read;

#[repr(u8)]
#[allow(dead_code)]
pub enum Operator {
    DecrementPointer, // >  -  move the pointer right
    IncrementPointer, // <  -  move the pointer left
    DecrementCell,    // +  -  increment the current cell
    IncrementCell,    // -  -  decrement the current cell
    Output,           // .  -  output the value of the current cell
    Input,            // ,  -  replace the value of the current cell with input
    LoopStart,        // [  -  jump to the matching ] instruction if the current value is zero
    LoopEnd,          // ]  -  jump to the matching [ instruction if the current value is not zero
}

#[allow(dead_code)]
pub struct VM {
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
