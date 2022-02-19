mod brainfuck;

fn main() {
    let hello_world = r#"++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>++.>+.+++++++..+++.<<++.>+++++++++++++++.>.+++.------.--------.<<+.<."#;

    let mut interpreter = brainfuck::Brainfuck::new();
    interpreter.run(hello_world.to_string());
}
