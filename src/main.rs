mod brainfuck;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename_or_program = args.get(1);
    if let None = filename_or_program {
        println!("usage: provide name of .bf file or a brainfuck program");
        return;
    }

    let filename_or_program = filename_or_program.unwrap().trim().to_string();

    let source = if filename_or_program.ends_with(".bf") {
        fs::read_to_string(filename_or_program).expect("error reading file")
    } else {
        filename_or_program
    };

    let mut interpreter = brainfuck::Brainfuck::new();
    interpreter.run(source);
}
