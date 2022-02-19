mod brainfuck;

fn main() {
    let mut program = Vec::new();

    for _ in 0..33 {
        program.push(brainfuck::Operator::IncrementCell);
    }

    for _ in 0..5 {
        program.push(brainfuck::Operator::Output);
    }

    let mut vm = brainfuck::VM::new();
    vm.run(&program);
}
