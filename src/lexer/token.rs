#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literales
    Number(i64),
    Float(f64),
    StringLiteral(String),
    Comment(String),

    // Identificadores y palabras clave
    Identifier(String),
    Let, Mut, If, Else, Match, While, Loop,
    Fn, Return, Break, Continue,
    Struct, Enum, Impl, Trait,
    Mod, Use, Const, Static, Async, Await,
    For, In, Pub, Crate, Super, SelfLower,
    SelfUpper, Type, Where, Move, Unsafe,

    // Tipos básicos
    Bool, Char, Str,
    U8, U16, U32, U64, Usize,
    I8, I16, I32, I64, Isize,
    F32, F64,

    // Operadores
    Assign,         // =
    Plus,           // +
    Minus,          // -
    Asterisk,       // *
    Slash,          // /
    Percent,        // %
    Caret,          // ^
    Not,            // !
    And,            // &&
    Or,             // ||
    BitAnd,         // &
    BitOr,          // |
    Shl,            // <<
    Shr,            // >>
    Equal,          // ==
    NotEqual,       // !=
    LessThan,       // <
    GreaterThan,    // >
    LessEqual,      // <=
    GreaterEqual,   // >=
    Arrow,          // =>
    FatArrow,       // ->

    // Separadores
    LParen, RParen,
    LBrace, RBrace,
    LBracket, RBracket,
    Semicolon, Colon, Comma, Dot, DoubleColon,

    // Error y fin
    Error(char),
    EOF,
}