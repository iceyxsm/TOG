use crate::error::TogError;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Int(i64),
    Float(f64),
    String(String),
    InterpolatedString(String), // String with {expr} interpolation
    Bool(bool),
    
    // Identifiers and keywords
    Identifier(String),
    Keyword(Keyword),
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,
    EqEq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Not,
    Dot,
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Colon,
    Arrow, // ->
    FatArrow, // =>
    
    // Other
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Fn,
    Let,
    Struct,
    If,
    Else,
    While,
    For,
    In,
    Return,
    Match,
    Break,
    Continue,
    None,
    Int,
    Float,
    String,
    Bool,
    Array,
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, TogError> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();
    let mut line = 1;
    let mut column = 1;
    
    while let Some(&ch) = chars.peek() {
        match ch {
            // Whitespace
            ' ' | '\t' => {
                chars.next();
                column += 1;
            }
            '\r' => {
                // Windows line ending - skip \r, \n will be handled next
                chars.next();
                column += 1;
            }
            '\n' => {
                chars.next();
                line += 1;
                column = 1;
            }
            
            // Comments
            '/' if matches!(chars.clone().nth(1), Some('/')) => {
                while let Some(&ch) = chars.peek() {
                    if ch == '\n' {
                        break;
                    }
                    chars.next();
                }
            }
            
            // Numbers
            '0'..='9' => {
                let _start_col = column;
                let mut num_str = String::new();
                let mut is_float = false;
                
                while let Some(&ch) = chars.peek() {
                    match ch {
                        '0'..='9' => {
                            num_str.push(ch);
                            chars.next();
                            column += 1;
                        }
                        '.' if !is_float => {
                            is_float = true;
                            num_str.push(ch);
                            chars.next();
                            column += 1;
                        }
                        _ => break,
                    }
                }
                
                if is_float {
                    let num = num_str.parse::<f64>()
                        .map_err(|_| TogError::LexError(
                            format!("Invalid float: {}", num_str),
                            line,
                            _start_col
                        ))?;
                    tokens.push(Token::Float(num));
                } else {
                    let num = num_str.parse::<i64>()
                        .map_err(|_| TogError::LexError(
                            format!("Invalid integer: {}", num_str),
                            line,
                            _start_col
                        ))?;
                    tokens.push(Token::Int(num));
                }
            }
            
            // Strings (with interpolation support)
            '"' => {
                let _start_col = column;
                chars.next(); // consume opening quote
                column += 1;
                let mut string = String::new();
                let mut has_interpolation = false;
                
                while let Some(ch) = chars.next() {
                    column += 1;
                    match ch {
                        '"' => break,
                        '{' => {
                            // String interpolation: {expr}
                            has_interpolation = true;
                            // For now, we'll handle this in the parser
                            // Just mark it and continue
                            string.push(ch);
                        }
                        '}' => {
                            string.push(ch);
                        }
                        '\\' => {
                            match chars.next() {
                                Some('n') => {
                                    string.push('\n');
                                    column += 1;
                                }
                                Some('t') => {
                                    string.push('\t');
                                    column += 1;
                                }
                                Some('\\') => {
                                    string.push('\\');
                                    column += 1;
                                }
                                Some('"') => {
                                    string.push('"');
                                    column += 1;
                                }
                                _ => return Err(TogError::LexError(
                                    "Invalid escape sequence".to_string(),
                                    line,
                                    column
                                )),
                            }
                        }
                        _ => string.push(ch),
                    }
                }
                
                if has_interpolation {
                    tokens.push(Token::InterpolatedString(string));
                } else {
                    tokens.push(Token::String(string));
                }
            }
            
            // Operators and punctuation
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
                column += 1;
            }
            '-' => {
                chars.next();
                column += 1;
                if matches!(chars.peek(), Some('>')) {
                    chars.next();
                    column += 1;
                    tokens.push(Token::Arrow);
                } else {
                    tokens.push(Token::Minus);
                }
            }
            '*' => {
                tokens.push(Token::Star);
                chars.next();
                column += 1;
            }
            '/' => {
                tokens.push(Token::Slash);
                chars.next();
                column += 1;
            }
            '%' => {
                tokens.push(Token::Percent);
                chars.next();
                column += 1;
            }
            '.' => {
                tokens.push(Token::Dot);
                chars.next();
                column += 1;
            }
            '=' => {
                chars.next();
                column += 1;
                if matches!(chars.peek(), Some('=')) {
                    chars.next();
                    column += 1;
                    tokens.push(Token::EqEq);
                } else if matches!(chars.peek(), Some('>')) {
                    chars.next();
                    column += 1;
                    tokens.push(Token::FatArrow);
                } else {
                    tokens.push(Token::Eq);
                }
            }
            '!' => {
                chars.next();
                column += 1;
                if matches!(chars.peek(), Some('=')) {
                    chars.next();
                    column += 1;
                    tokens.push(Token::Ne);
                } else {
                    tokens.push(Token::Not);
                }
            }
            '<' => {
                chars.next();
                column += 1;
                if matches!(chars.peek(), Some('=')) {
                    chars.next();
                    column += 1;
                    tokens.push(Token::Le);
                } else {
                    tokens.push(Token::Lt);
                }
            }
            '>' => {
                chars.next();
                column += 1;
                if matches!(chars.peek(), Some('=')) {
                    chars.next();
                    column += 1;
                    tokens.push(Token::Ge);
                } else {
                    tokens.push(Token::Gt);
                }
            }
            '&' => {
                chars.next();
                column += 1;
                if matches!(chars.peek(), Some('&')) {
                    chars.next();
                    column += 1;
                    tokens.push(Token::And);
                } else {
                    return Err(TogError::LexError(
                        "Unexpected '&'".to_string(),
                        line,
                        column
                    ));
                }
            }
            '|' => {
                chars.next();
                column += 1;
                if matches!(chars.peek(), Some('|')) {
                    chars.next();
                    column += 1;
                    tokens.push(Token::Or);
                } else {
                    return Err(TogError::LexError(
                        "Unexpected '|'".to_string(),
                        line,
                        column
                    ));
                }
            }
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
                column += 1;
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
                column += 1;
            }
            '{' => {
                tokens.push(Token::LeftBrace);
                chars.next();
                column += 1;
            }
            '}' => {
                tokens.push(Token::RightBrace);
                chars.next();
                column += 1;
            }
            '[' => {
                tokens.push(Token::LeftBracket);
                chars.next();
                column += 1;
            }
            ']' => {
                tokens.push(Token::RightBracket);
                chars.next();
                column += 1;
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
                column += 1;
            }
            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
                column += 1;
            }
            ':' => {
                tokens.push(Token::Colon);
                chars.next();
                column += 1;
            }
            
            // Identifiers and keywords
            ch if ch.is_alphabetic() || ch == '_' => {
                let _start_col = column;
                let mut ident = String::new();
                
                while let Some(&ch) = chars.peek() {
                    match ch {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                            ident.push(ch);
                            chars.next();
                            column += 1;
                        }
                        _ => break,
                    }
                }
                
                // Check if it's a keyword
                match ident.as_str() {
                    "fn" => tokens.push(Token::Keyword(Keyword::Fn)),
                    "let" => tokens.push(Token::Keyword(Keyword::Let)),
                    "struct" => tokens.push(Token::Keyword(Keyword::Struct)),
                    "if" => tokens.push(Token::Keyword(Keyword::If)),
                    "else" => tokens.push(Token::Keyword(Keyword::Else)),
                    "while" => tokens.push(Token::Keyword(Keyword::While)),
                    "for" => tokens.push(Token::Keyword(Keyword::For)),
                    "in" => tokens.push(Token::Keyword(Keyword::In)),
                    "return" => tokens.push(Token::Keyword(Keyword::Return)),
                    "match" => tokens.push(Token::Keyword(Keyword::Match)),
                    "break" => tokens.push(Token::Keyword(Keyword::Break)),
                    "continue" => tokens.push(Token::Keyword(Keyword::Continue)),
                    "none" => tokens.push(Token::Keyword(Keyword::None)),
                    "true" => tokens.push(Token::Bool(true)),
                    "false" => tokens.push(Token::Bool(false)),
                    // "print" is now a built-in function, not a keyword
                    // "print" => tokens.push(Token::Keyword(Keyword::Print)),
                    "int" => tokens.push(Token::Keyword(Keyword::Int)),
                    "float" => tokens.push(Token::Keyword(Keyword::Float)),
                    "string" => tokens.push(Token::Keyword(Keyword::String)),
                    "bool" => tokens.push(Token::Keyword(Keyword::Bool)),
                    "array" => tokens.push(Token::Keyword(Keyword::Array)),
                    _ => tokens.push(Token::Identifier(ident)),
                }
            }
            
            _ => {
                return Err(TogError::LexError(
                    format!("Unexpected character: {}", ch),
                    line,
                    column
                ));
            }
        }
    }
    
    tokens.push(Token::Eof);
    Ok(tokens)
}

