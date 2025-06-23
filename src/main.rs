mod lexer;
mod parser;

use lexer::{Lexer, Token};
use parser::{Parser, Stmt};

fn main() {
    let input = "
        let x = 10;
        if x > 5 {
            return x;
        } else {
            return 0;
        }
        while x < 100 {
            x = x + 1;
        }
        loop {
            return;
        }
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();

    println!("Tokens:");
    for token in &tokens {
        println!("{:?}", token);
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    println!("\nAST:");
    for stmt in ast {
        println!("{:#?}", stmt);
    }
}
