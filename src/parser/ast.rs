#[allow(dead_code)]
use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Identifier(String),
    StringLiteral(String),
    Float(f64),
    BinaryOp {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Call {
        function: String,
        argument: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    LetStmt {
        name: String,
        value: Expr,
    },
    ExprStmt(Expr),
    IfStmt {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    WhileStmt {
        condition: Expr,
        body: Vec<Stmt>,
    },
    LoopStmt {
        body: Vec<Stmt>,
    },
    ReturnStmt(Option<Expr>),
    BlockStmt(Vec<Stmt>), 
}
