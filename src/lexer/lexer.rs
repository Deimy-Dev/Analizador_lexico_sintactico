use crate::lexer::token::Token;
use crate::lexer::keywords::lookup_keyword;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

      pub fn next_token(&mut self) -> Token {
        while let Some(ch) = self.current_char() {
            match ch {
                ' ' | '\n' | '\t' => self.advance(),
                '+' => {
                    self.advance();
                    return Token::Plus;
                }
                '*' => {
                    self.advance();
                    return Token::Asterisk;
                }
                '(' => {
                    self.advance();
                    return Token::LParen;
                }
                ')' => {
                    self.advance();
                    return Token::RParen;
                }
                '[' => {
                    self.advance();
                    return Token::LBracket;
                }
                ']' => {
                    self.advance();
                    return Token::RBracket;
                }
                '{' => { 
                    self.advance(); 
                    return Token::LBrace; 
                }
                '}' => { 
                    self.advance(); 
                    return Token::RBrace; 
                }
                ';' => { 
                    self.advance(); 
                    return Token::Semicolon; 
                }
                ',' => { 
                    self.advance(); 
                    return Token::Comma; 
                }
                ':' => { 
                    self.advance(); 
                    return Token::Colon; 
                }
                '-' => {
                    if self.peek_char() == Some('>') {
                        self.advance();
                        self.advance();
                        return Token::FatArrow;
                    } else {
                        self.advance();
                        return Token::Minus;
                    }
                }
                '=' => {
                    if self.peek_char() == Some('=') {
                        self.advance(); self.advance();
                        return Token::Equal; // ==
                    } else if self.peek_char() == Some('>') {
                        self.advance(); self.advance();
                        return Token::Arrow; // =>
                    } else {
                        self.advance();
                        return Token::Assign; // =
                    }
                },
                '!' => {
                    if self.peek_char() == Some('=') {
                        self.advance(); self.advance();
                        return Token::NotEqual; // != (debes agregar NotEqual a Token enum)
                    } else {
                        panic!("Carácter no válido: !");
                    }
                },
                '<' => {
                    if self.peek_char() == Some('=') {
                        self.advance(); self.advance();
                        return Token::LessEqual; // <=
                    } else {
                        self.advance();
                        return Token::LessThan; // <
                    }
                },
                '>' => {
                    if self.peek_char() == Some('=') {
                        self.advance(); self.advance();
                        return Token::GreaterEqual; // >=
                    } else {
                        self.advance();
                        return Token::GreaterThan; // >
                    }
                },
                '&' => {
                    if self.peek_char() == Some('&') {
                        self.advance(); self.advance();
                        return Token::And; // &&
                    } else {
                        panic!("Carácter no válido: &");
                    }
                },
                '|' => {
                    if self.peek_char() == Some('|') {
                        self.advance(); self.advance();
                        return Token::Or; // ||
                    } else {
                        panic!("Carácter no válido: |");
                    }
                },
                '0'..='9' => {
                    let start = self.position;
                    let mut has_dot = false;

                    while let Some(c) = self.current_char() {
                        if c.is_numeric() {
                            self.advance();
                        } else if c == '.' && !has_dot {
                            // Mira si el siguiente también es dígito para evitar token como "3." solo.
                            if let Some(next_c) = self.input.get(self.position + 1) {
                                if next_c.is_numeric() {
                                    has_dot = true;
                                    self.advance();
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }

                    let number_str: String = self.input[start..self.position].iter().collect();

                    // Ahora diferenciar entre entero y decimal:
                    if has_dot {
                        // Token para número decimal - puedes crear otro enum Token::Float(f64)
                        return Token::Float(number_str.parse().unwrap());
                    } else {
                        return Token::Number(number_str.parse().unwrap());
                    }
                }

                'a'..='z' | 'A'..='Z' => {
                    let start = self.position;
                    while let Some(c) = self.current_char() {
                        if c.is_alphanumeric() {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    let ident: String = self.input[start..self.position].iter().collect();
                    return lookup_keyword(&ident);
                }
                '/' => {
                    if self.peek_char() == Some('/') {
                        // Comentario de línea
                        self.advance(); // consume primer '/'
                        self.advance(); // consume segundo '/'

                        let start = self.position;
                        while let Some(c) = self.current_char() {
                            if c == '\n' {
                                break;
                            }
                            self.advance();
                        }

                        let comment: String = self.input[start..self.position].iter().collect();
                        return Token::Comment(comment);
                    } else if self.peek_char() == Some('*') {
                        self.advance(); // consume '/'
                        self.advance(); // consume '*'

                        let start = self.position;
                        while let Some(c) = self.current_char() {
                            if c == '*' && self.peek_char() == Some('/') {
                                self.advance(); // consume '*'
                                self.advance(); // consume '/'
                                break;
                            }
                            self.advance();
                        }
                        let comment: String = self.input[start..self.position - 2].iter().collect(); // -2 para quitar */ final
                        return Token::Comment(comment);
                    }
                    else {
                        self.advance();
                        return Token::Slash;
                    }
                }
                '"' | '\'' => {
                    let quote = ch;
                    self.advance();
                    let start = self.position;
                    while let Some(c) = self.current_char() {
                        if c == quote {
                            break;
                        }
                        self.advance();
                    }
                    let literal: String = self.input[start..self.position].iter().collect();
                    self.advance(); // Consumir cierre de comillas
                    return Token::StringLiteral(literal);
                }
                _ => {
                    let err = ch;
                    self.advance();
                    return Token::Error(err);
                }

            }
        }
        
        Token::EOF
    }
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            if let Token::EOF = token {
                tokens.push(token);
                break;
            } else {
                tokens.push(token);
            }
        }
        tokens
    }
}