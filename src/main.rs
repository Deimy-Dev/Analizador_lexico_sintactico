mod lexer;
mod parser;

use lexer::{Lexer, Token};
use parser::{Parser, Stmt};

fn main() {
    let input = "
if x > 5 {
    return x;
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

    println!("\nÁrbol de Sintaxis Abstracta:");
    for stmt in ast {
        println!("{:#?}", stmt);
        println!(" ");
    }
}
