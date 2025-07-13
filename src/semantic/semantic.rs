use crate::parser::ast::{Expr, Stmt};
use crate::parser::ast::Expr::*;
use crate::parser::ast::Stmt::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    String,
    Unknown,
}

#[derive(Debug)]
struct SymbolInfo {
    ty: Type,
    line: usize,
}

pub struct SemanticAnalyzer {
    symbols: HashMap<String, SymbolInfo>,
    warnings: Vec<String>,
    errors: Vec<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn analyze(&mut self, ast: &[Stmt]) {
        for stmt in ast {
            self.visit_stmt(stmt, 1);
        }

        for w in &self.warnings {
            println!("⚠ Advertencia: {}", w);
        }

        for e in &self.errors {
            println!("🛑 Error: {}", e);
        }

        println!("\nTabla de símbolos:");
        for (name, info) in &self.symbols {
            println!(" - {}: {:?} (línea {})", name, info.ty, info.line);
        }

        println!(
            "\nAnálisis completado: {} advertencia{}, {} error{}.",
            self.warnings.len(),
            if self.warnings.len() == 1 { "" } else { "s" },
            self.errors.len(),
            if self.errors.len() == 1 { "" } else { "es" }
        );
    }

    fn visit_stmt(&mut self, stmt: &Stmt, line: usize) {
        match stmt {
            LetStmt { name, value } => {
                let ty = self.infer_expr_type(value);
                self.symbols.insert(name.clone(), SymbolInfo { ty, line });
            }
            IfStmt { condition, then_branch, else_branch } => {
                self.visit_expr(condition, line);
                for stmt in then_branch {
                    self.visit_stmt(stmt, line);
                }
                if let Some(branch) = else_branch {
                    for stmt in branch {
                        self.visit_stmt(stmt, line);
                    }
                }
            }
            WhileStmt { condition, body } => {
                self.visit_expr(condition, line);
                for stmt in body {
                    self.visit_stmt(stmt, line);
                }
            }
            LoopStmt { body } => {
                for stmt in body {
                    self.visit_stmt(stmt, line);
                }
            }
            ExprStmt(expr) => {
                self.visit_expr(expr, line);
            }
            ReturnStmt(Some(expr)) => {
                self.visit_expr(expr, line);
            }
            _ => {}
        }
    }

    fn visit_expr(&mut self, expr: &Expr, line: usize) -> Type {
        match expr {
            Number(_) => Type::Int,
            Float(_) => Type::Float,
            StringLiteral(_) => Type::String,
            Identifier(name) => {
                self.symbols.get(name).map_or(Type::Unknown, |info| info.ty.clone())
            }
            BinaryOp { left, op, right } => {
                let left_type = self.visit_expr(left, line);
                let right_type = self.visit_expr(right, line);

                if left_type != right_type {
                    self.warnings.push(format!(
                        "Comparación entre tipos distintos en línea {}: '{:?}' ({:?}) {:?} '{:?}' ({:?})",
                        line,
                        left,
                        left_type,
                        op,
                        right,
                        right_type
                    ));
                }

                if left_type == right_type {
                    left_type
                } else {
                    Type::Unknown
                }
            }
            Call { function: _, argument } => {
                self.visit_expr(argument, line);
                Type::Unknown
            }
        }
    }

    fn infer_expr_type(&self, expr: &Expr) -> Type {
        match expr {
            Number(_) => Type::Int,
            Float(_) => Type::Float,
            StringLiteral(_) => Type::String,
            Identifier(name) => {
                self.symbols.get(name).map_or(Type::Unknown, |info| info.ty.clone())
            }
            _ => Type::Unknown,
        }
    }
}
