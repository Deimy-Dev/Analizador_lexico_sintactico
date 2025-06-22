use std::collections::HashMap;

use super::token::Token;

/// Retorna un `Token` si el identificador es una palabra clave.
pub fn lookup_keyword(ident: &str) -> Token {
    KEYWORDS.get(ident).cloned().unwrap_or(Token::Identifier(ident.to_string()))
}

lazy_static::lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, Token> = {
        let mut m = HashMap::new();

        // Palabras clave tomadas de Rust
        m.insert("let", Token::Let);
        m.insert("mut", Token::Mut);
        m.insert("if", Token::If);
        m.insert("else", Token::Else);
        m.insert("match", Token::Match);
        m.insert("while", Token::While);
        m.insert("loop", Token::Loop);
        m.insert("fn", Token::Fn);
        m.insert("return", Token::Return);
        m.insert("break", Token::Break);
        m.insert("continue", Token::Continue);
        m.insert("struct", Token::Struct);
        m.insert("enum", Token::Enum);
        m.insert("impl", Token::Impl);
        m.insert("trait", Token::Trait);
        m.insert("mod", Token::Mod);
        m.insert("use", Token::Use);
        m.insert("const", Token::Const);
        m.insert("static", Token::Static);
        m.insert("async", Token::Async);
        m.insert("await", Token::Await);
        m.insert("for", Token::For);
        m.insert("in", Token::In);
        m.insert("pub", Token::Pub);
        m.insert("crate", Token::Crate);
        m.insert("super", Token::Super);
        m.insert("self", Token::SelfLower);
        m.insert("Self", Token::SelfUpper);
        m.insert("type", Token::Type);
        m.insert("where", Token::Where);
        m.insert("move", Token::Move);
        m.insert("unsafe", Token::Unsafe);
        m
    };
}
