mod lexer;
mod token;

use std::io::{self, Write};

fn main() {
    println!("READY.");

    loop {
        print!(":");
        io::stdout().flush().unwrap();

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
