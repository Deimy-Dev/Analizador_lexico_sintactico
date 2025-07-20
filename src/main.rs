mod lexer;
mod parser;
mod semantic;
mod codegen; // recuerda crear este módulo para generación de código

use lexer::Lexer;
use parser::Parser;
use semantic::SemanticAnalyzer;
use codegen::generate_cpp;

fn main() {
    let input = "
    let a = 5;
let b = 3.2;
let c = a + b;
print(c);

    ";

    // Lexer
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();

    println!("\nTokens:");
    for token in &tokens {
        println!("{:?}", token);
    }

    // Parser
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    println!("\nÁrbol de Sintaxis Abstracta:");
    for stmt in &ast {
        println!("{:#?}", stmt);
        println!(" ");
    }

    // Análisis semántico
    println!("\n--- Análisis Semántico ---");
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&ast);

    // Generación de código C++
    println!("\n--- Código generado en C++ ---");
    let cpp_code = generate_cpp(&ast);
    println!("{}", cpp_code);
}
