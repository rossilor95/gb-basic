mod lexer;
mod token;

use std::io;

fn main() {
    println!("READY.");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let lexer = lexer::Lexer::new(&input);

        for token in lexer {
            println!("{:?}", token);
        }
    }
}
