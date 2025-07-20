use crate::parser::ast::{Expr, Stmt};
use crate::lexer::Token;
use crate::semantic::semantic::Type;


pub fn generate_cpp(ast: &Vec<Stmt>) -> String {
    let mut output = String::new();

    // Cabecera de C++
    output.push_str("#include <iostream>\nusing namespace std;\n\nint main() {\n");

    for stmt in ast {
        output.push_str(&format!("  {}\n", gen_stmt(stmt)));
    }

    // return 0 al final del main
    output.push_str("  return 0;\n}\n");

    output
}

fn gen_stmt(stmt: &Stmt) -> String {
    match stmt {
        Stmt::LetStmt { name, value } => {
            let ty = infer_type(value);
            let ty_str = match ty {
                Type::Int => "int",
                Type::Float => "float",
                _ => "auto", // default
            };
            format!("{} {} = {};", ty_str, name, gen_expr(value))
        }
        Stmt::ExprStmt(expr) => {
            match expr {
                Expr::Call { function, argument } if function == "print" => {
                    format!("cout << {} << endl;", gen_expr(argument))
                }
                _ => format!("{};", gen_expr(expr)),
            }
        }
        Stmt::IfStmt { condition, then_branch, else_branch } => {
            let mut s = format!("if ({}) {{\n", gen_expr(condition));
            for stmt in then_branch {
                s.push_str(&format!("  {}\n", gen_stmt(stmt)));
            }
            s.push_str("}");
            if let Some(else_branch) = else_branch {
                s.push_str(" else {\n");
                for stmt in else_branch {
                    s.push_str(&format!("  {}\n", gen_stmt(stmt)));
                }
                s.push_str("}");
            }
            s
        }
        Stmt::WhileStmt { condition, body } => {
            let mut s = format!("while ({}) {{\n", gen_expr(condition));
            for stmt in body {
                s.push_str(&format!("  {}\n", gen_stmt(stmt)));
            }
            s.push_str("}");
            s
        }
        Stmt::LoopStmt { body } => {
            let mut s = "while (true) {\n".to_string();
            for stmt in body {
                s.push_str(&format!("  {}\n", gen_stmt(stmt)));
            }
            s.push_str("}");
            s
        }
        Stmt::ReturnStmt(Some(expr)) => {
            format!("return {};", gen_expr(expr))
        }
        Stmt::ReturnStmt(None) => {
            "return;".to_string()
        }
        _ => "// Unsupported statement".to_string(),
    }
}

fn gen_expr(expr: &Expr) -> String {
    match expr {
        Expr::Number(n) => n.to_string(),
        Expr::Float(f) => f.to_string(),
        Expr::StringLiteral(s) => format!("\"{}\"", s),
        Expr::Identifier(name) => name.clone(),
        Expr::BinaryOp { left, op, right } => {
            let op_str = match op {
                Token::Plus => "+",
                Token::Minus => "-",
                Token::Equal => "==",
                Token::LessThan => "<",
                Token::Assign => "=",
                Token::NotEqual => "!=",
                // Añade todos los operadores que uses
                _ => "<op>",
            };
            format!("{} {} {}", gen_expr(left), op_str, gen_expr(right))
        }
        _ => "<expr unsupported>".to_string(),
    }
}

fn infer_type(expr: &Expr) -> Type {
    match expr {
        Expr::Number(_) => Type::Int,
        Expr::Float(_) => Type::Float,
        _ => Type::Unknown,
    }
}


fn gen_op(op: &Token) -> &'static str {
    match op {
        Token::Plus => "+",
        Token::Minus => "-",
        Token::Asterisk => "*",
        Token::Slash => "/",
        Token::Equal => "==",
        Token::NotEqual => "!=",
        Token::LessThan => "<",
        Token::GreaterThan => ">",
        // Completa según tus tokens
        _ => "<op_no_soportado>",
    }
}
