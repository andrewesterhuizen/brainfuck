mod brainfuck;

fn main() {
    // print "!!!!!"
    let mut program = String::new();
    program += &"+".repeat(33);
    program += &".".repeat(5);

    let mut interpreter = brainfuck::Brainfuck::new();
    interpreter.run(program);
}
