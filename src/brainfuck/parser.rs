use crate::brainfuck::operator::Operator;

pub struct Parser {}

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
