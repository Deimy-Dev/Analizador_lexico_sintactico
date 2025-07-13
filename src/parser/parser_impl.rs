use crate::lexer::Token;
use crate::parser::ast::{Expr, Stmt};
use std::mem::discriminant;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn match_token(&self, expected: &Token) -> bool {
        discriminant(self.current()) == discriminant(expected)
    }

    fn consume(&mut self, expected: &Token) -> bool {
        if self.match_token(expected) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    fn skip_comments(&mut self) {
        while matches!(self.current(), Token::Comment(_)) {
            self.advance();
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while !self.match_token(&Token::EOF) {
            self.skip_comments();  // Salta comentarios 
            match self.parse_stmt() {
                Some(stmt) => stmts.push(stmt),
                None => {
                    println!("Saltando token: {:?}", self.current());
                    self.advance();
                }
            }
        }
        stmts
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        self.skip_comments();  // Salta comentarios

        match self.current() {
            Token::Let => self.parse_let_stmt(),
            Token::If => self.parse_if_stmt(),
            Token::While => self.parse_while_stmt(),
            Token::Loop => self.parse_loop_stmt(),
            Token::Return => self.parse_return_stmt(),
            Token::LBrace => {
                let block_stmts = self.parse_block()?;
                Some(Stmt::BlockStmt(block_stmts))
            }
            _ => self.parse_expr_stmt(),
        }
    }


    fn parse_let_stmt(&mut self) -> Option<Stmt> {
        self.advance(); // consume 'let'
        let name = if let Token::Identifier(name) = self.current().clone() {
            self.advance();
            name
        } else {
            println!("Error: se esperaba un identificador después de 'let'");
            return None;
        };

        if !self.consume(&Token::Assign) {
            println!("Error: se esperaba '=' después del identificador");
            return None;
        }

        let expr = self.parse_expression()?;
        if !self.consume(&Token::Semicolon) {
            println!("Error: se esperaba ';' después de la expresión");
            return None;
        }

        Some(Stmt::LetStmt { name, value: expr })
    }

    fn parse_if_stmt(&mut self) -> Option<Stmt> {
        self.advance(); // consume 'if'
        let condition = self.parse_expression()?;

        let then_branch = self.parse_block()?;

        let else_branch = if self.match_token(&Token::Else) {
            self.advance();
            Some(self.parse_block()?)
        } else {
            None
        };

        Some(Stmt::IfStmt {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_while_stmt(&mut self) -> Option<Stmt> {
        self.advance(); // consume 'while'
        let condition = self.parse_expression()?;
        let body = self.parse_block()?;
        Some(Stmt::WhileStmt { condition, body })
    }

    fn parse_loop_stmt(&mut self) -> Option<Stmt> {
        self.advance(); // consume 'loop'
        let body = self.parse_block()?;
        Some(Stmt::LoopStmt { body })
    }

    fn parse_return_stmt(&mut self) -> Option<Stmt> {
        self.advance(); // consume 'return'
        let expr = if self.match_token(&Token::Semicolon) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        if !self.consume(&Token::Semicolon) {
            println!("Error: se esperaba ';' después de return");
            return None;
        }

        Some(Stmt::ReturnStmt(expr))
    }

    fn parse_expr_stmt(&mut self) -> Option<Stmt> {
        let expr = self.parse_expression()?;
        if !self.consume(&Token::Semicolon) {
            println!("Error: se esperaba ';'");
            return None;
        }
        Some(Stmt::ExprStmt(expr))
    }


    fn parse_block(&mut self) -> Option<Vec<Stmt>> {
        if !self.consume(&Token::LBrace) {
            println!("Error: se esperaba '{{' para iniciar bloque, pero se encontró {:?}", self.current());
            return None;
        }

        let mut stmts = Vec::new();
        while !self.match_token(&Token::RBrace) && !self.match_token(&Token::EOF) {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            } else {
                println!("Saltando token inválido en bloque: {:?}", self.current());
                self.advance();
            }
        }

        if !self.consume(&Token::RBrace) {
            println!("Error: se esperaba '}}' para cerrar bloque");
            return None;
        }

        Some(stmts)
    }

    // ------------ Expresiones ------------

    fn parse_expression(&mut self) -> Option<Expr> {
        self.parse_assignment()
    }


    fn parse_assignment(&mut self) -> Option<Expr> {
        let expr = self.parse_equality()?; // empieza con igualdad

        if self.match_token(&Token::Assign) {
            self.advance(); // consume '='
            let value = self.parse_assignment()?; // recursividad para encadenar asignaciones
            if let Expr::Identifier(name) = expr {
                return Some(Expr::BinaryOp {
                    left: Box::new(Expr::Identifier(name)),
                    op: Token::Assign,
                    right: Box::new(value),
                });
            } else {
                println!("Error: la parte izquierda de una asignación debe ser un identificador");
                return None;
            }
        }

        Some(expr)
    }


    fn parse_equality(&mut self) -> Option<Expr> {
        let mut expr = self.parse_comparison()?;

        while matches!(self.current(), Token::Equal | Token::NotEqual) {
            let op = self.current().clone();
            self.advance();
            let right = self.parse_comparison()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Some(expr)
    }

    fn parse_comparison(&mut self) -> Option<Expr> {
        let mut expr = self.parse_term()?;

        while matches!(
            self.current(),
            Token::LessThan | Token::LessEqual | Token::GreaterThan | Token::GreaterEqual
        ) {
            let op = self.current().clone();
            self.advance();
            let right = self.parse_term()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Some(expr)
    }

    fn parse_term(&mut self) -> Option<Expr> {
        let mut expr = self.parse_factor()?;

        while matches!(self.current(), Token::Plus | Token::Minus) {
            let op = self.current().clone();
            self.advance();
            let right = self.parse_factor()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Some(expr)
    }

    fn parse_factor(&mut self) -> Option<Expr> {
        let mut expr = self.parse_primary()?;

        while matches!(self.current(), Token::Asterisk | Token::Slash) {
            let op = self.current().clone();
            self.advance();
            let right = self.parse_primary()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Some(expr)
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        match self.current() {
            Token::Number(n) => {
                let val = *n;
                self.advance();
                Some(Expr::Number(val))
            }
            Token::Float(f) => {
                let val = *f;      
                self.advance();    
                Some(Expr::Float(val))
            }
            Token::StringLiteral(s) => {
                let val = s.clone();
                self.advance();
                Some(Expr::StringLiteral(val))  
            }
            Token::Identifier(name) => {
                let id = name.clone();
                self.advance();

                if self.consume(&Token::LParen) {
                    let arg = self.parse_expression()?;
                    if !self.consume(&Token::RParen) {
                        println!("Error: se esperaba ')' después de argumentos");
                        return None;
                    }
                    return Some(Expr::Call {
                        function: id,
                        argument: Box::new(arg),
                    });
                }

                Some(Expr::Identifier(id))
            }


            Token::LParen => {
                self.advance();
                let expr = self.parse_expression();
                self.consume(&Token::RParen);
                expr
            }
            _ => {
                println!("Error: token inesperado {:?}", self.current());
                None
            }
        }
    }
}
