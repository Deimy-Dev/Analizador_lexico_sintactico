mod lexer;

use lexer::{Lexer, Token};

fn main() {
    let input = r#"
fn suma(a: i32, b: i32) -> i32 {
    if a >= b && b != 0 {
        return a + b;
    } else {
        return a - b;
    }
}

// Comentario de línea

/* Comentario
   multilínea */

let mensaje = "Hola mundo!";
const PI: f64 = 3.1415;
let arreglo = [1, 2, 3];
"#;

    let mut lexer = Lexer::new(input);

    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        if token == Token::EOF {
            break;
        }
        tokens.push(token);
    }

    println!("Tokens:");
    for token in &tokens {
        println!("{:?}", token);
    }
}
