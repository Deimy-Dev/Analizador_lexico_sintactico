//Deimy Minaya 1-21-0568 
mod lexer;
mod parser;
mod semantic;

use lexer::Lexer;
use parser::Parser;
use semantic::SemanticAnalyzer;


fn main() {
    let input = "
    // inicialización
    /* variables */
    let y = 20.5;
    let z = 10;

    if (y < z) {
        print('Menor');
    } else {
        print('Mayor o igual');
    }

    while z != 0 {
        z = z - 1;
    }

    loop {
        return y;
    }
        ";


    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    println!("");
    println!("");
    println!("");
    println!("Tokens:");
    for token in &tokens {
        println!("{:?}", token);
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    println!("\nÁrbol de Sintaxis Abstracta:");
    for stmt in &ast {
        println!("{:#?}", stmt);
        println!(" ");
    }

    println!("\n--- Análisis Semántico ---");
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&ast);
    println!("");

}









/*
//comentario
    /*hola
    hi*/
    let x = 10;
    if (x == 10.5) {
        print(\"Iguales\");
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
*/