use std::{ iter, vec };

pub struct Tokenizer {
    tokens: Vec<Token>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    String(String),
    Float(f64),
    Int(i64),
    Bool(bool),
    List(Vec<Value>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    EndLine,
    Break,
    Do,
    Else,
    ElseIf,
    End,
    False,
    True,
    For,
    Function,
    If,
    In,
    Local,
    Nil,
    Not,
    Repeat,
    Return,
    Then,
    Until,
    While,
    Operator(Operator),
    OperatorAssign(Operator),
    Len,
    Set,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    OpenSquare,
    CloseSquare,
    Semicolon,
    Colon,
    Comma,
    Dot,
    TripleDot,
    Apostrophe,
    VariableOrFunction(String),
    Value(Value),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    FloorDivide,
    Mod,
    Power,
    Concatenation,
    Relational(Comparison),
    Equals,
    NotEquals,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Comparison {
    Less,
    LessOrEqual,
    More,
    MoreOrEqual,
}

const SEPERATORS: &'static [&str] = &[" ", "\n", "\t", "\r"];
const OPERATORS: &'static [&str] = &["!", "+", "-", "*", "/", "%", "^", "<", ">", "="];
const NON_EXTENDABLE: &'static [&str] = &[")", "(", ",", "[", "]"];
impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer { tokens: vec![] }
    }
    pub fn tokenize(&mut self, input: String) {
        let mut buf = String::new();

        let mut iterator = input.chars().peekable();
        'char_iter: while let Some(c) = iterator.next() {
            if c == '-' && *iterator.peek().unwrap_or(&'x') == '-' {
                iterator.next();
                self.add_token(Tokenizer::try_match_token(&buf));
                buf.clear();
                let x = iterator.next().unwrap_or('x');
                if x == '[' && *iterator.peek().unwrap_or(&'x') == '[' {
                    iterator.next();
                    while let Some(c) = iterator.next() {
                        if c == '-' && *iterator.peek().unwrap_or(&'x') == '-' {
                            iterator.next();
                            let x = iterator.next().unwrap_or('x');
                            if x == ']' && *iterator.peek().unwrap_or(&'x') == ']' {
                                iterator.next();

                                continue 'char_iter;
                            }
                        }
                    }
                    // End of file
                    // Probably should throw error: Unmatched block comment
                    buf.clear();
                    continue 'char_iter; // Break should probably do the same
                } else {
                    while let Some(c) = iterator.next() {
                        if c == '\n' {
                            buf.clear();
                            continue 'char_iter;
                        }
                    }
                }
                continue 'char_iter;
            }
            if SEPERATORS.contains(&c.to_string().as_str()) {
                self.add_token(Tokenizer::try_match_token(&buf));
                buf.clear();
                if c == '\n' {
                    self.add_token(Some(Token::EndLine));
                }
                continue;
            }
            if OPERATORS.contains(&c.to_string().as_str()) {
                self.add_token(Tokenizer::try_match_token(&buf));
                buf.clear();
                buf.push(c);
                //x+2
                while let Some(c) = iterator.peek() {
                    if
                        (((*c == '/' && buf == "/") || *c == '=') && buf.len() == 1) ||
                        (buf == "//" && *c == '=')
                    {
                        buf.push(*c);
                        iterator.next();
                    } else {
                        break;
                    }
                }

                self.add_token(Tokenizer::try_match_token(&buf));
                eprintln!("Buf: {}", buf);
                buf.clear();

                continue;
            }
            if c == '\"' {
                self.add_token(Tokenizer::try_match_token(&buf));
                buf.clear();
                buf.push(c);
                while let Some(c) = iterator.next() {
                    buf.push(c);
                    if c == '\"' {
                        self.add_token(Tokenizer::try_match_token(&buf));
                        buf.clear();
                        break;
                    }
                }
                continue;
            }
            if NON_EXTENDABLE.contains(&c.to_string().as_str()) {
                self.add_token(Tokenizer::try_match_token(&buf));
                buf.clear();
                buf.push(c);
                self.add_token(Tokenizer::try_match_token(&buf));
                buf.clear();
                continue;
            }

            buf.push(c);
        }
        self.add_token(Tokenizer::try_match_token(&buf));
    }

    pub fn get_tokens(&self) -> &[Token] {
        &self.tokens
    }

    fn try_match_token(token: &str) -> Option<Token> {
        if let Some(t) = Tokenizer::get_token_from_keyword(token) {
            return Some(t);
        }
        if let Ok(t) = token.parse::<i64>() {
            return Some(Token::Value(Value::Int(t)));
        }
        if let Ok(t) = token.parse::<f64>() {
            return Some(Token::Value(Value::Float(t)));
        }

        if token.starts_with("\"") && token.ends_with("\"") {
            return Some(Token::Value(Value::String(String::from(token))));
        }
        if token.len() > 0 {
            return Some(Token::VariableOrFunction(token.to_string()));
        }
        None
    }

    fn add_token(&mut self, token: Option<Token>) {
        if let Some(token) = token {
            self.tokens.push(token);
        }
    }

    fn get_token_from_keyword(buf: &str) -> Option<Token> {
        // TODO: Should probably make a hashmap
        return match buf {
            "{" => Some(Token::OpenCurly),
            "}" => Some(Token::CloseCurly),
            "[" => Some(Token::OpenSquare),
            "]" => Some(Token::CloseSquare),
            "(" => Some(Token::OpenParen),
            ")" => Some(Token::CloseParen),
            "nil" => Some(Token::Nil),
            "break" => Some(Token::Break),
            "and" => Some(Token::Operator(Operator::And)),
            "or" => Some(Token::Operator(Operator::Or)),
            "function" => Some(Token::Function),
            "local" => Some(Token::Local),
            "in" => Some(Token::In),
            "+" => Some(Token::Operator(Operator::Add)),
            "-" => Some(Token::Operator(Operator::Subtract)),
            "/" => Some(Token::Operator(Operator::Divide)),
            "//" => Some(Token::Operator(Operator::FloorDivide)),
            "*" => Some(Token::Operator(Operator::Multiply)),
            "%" => Some(Token::Operator(Operator::Mod)),
            "^" => Some(Token::Operator(Operator::Power)),
            "+=" => Some(Token::OperatorAssign(Operator::Add)),
            "-=" => Some(Token::OperatorAssign(Operator::Subtract)),
            "/=" => Some(Token::OperatorAssign(Operator::Divide)),
            "//=" => Some(Token::OperatorAssign(Operator::FloorDivide)),
            "*=" => Some(Token::OperatorAssign(Operator::Multiply)),
            "%=" => Some(Token::OperatorAssign(Operator::Mod)),
            "^=" => Some(Token::OperatorAssign(Operator::Power)),
            ";" => Some(Token::Semicolon),
            "." => Some(Token::Dot),
            "," => Some(Token::Comma),
            ":" => Some(Token::Colon),
            ".." => Some(Token::Operator(Operator::Concatenation)),
            "..." => Some(Token::TripleDot),
            "=" => Some(Token::Set),
            "==" => Some(Token::Operator(Operator::Equals)),
            "~=" => Some(Token::Operator(Operator::NotEquals)),
            "<" => Some(Token::Operator(Operator::Relational(Comparison::Less))),
            ">" => Some(Token::Operator(Operator::Relational(Comparison::More))),
            "<=" => Some(Token::Operator(Operator::Relational(Comparison::LessOrEqual))),
            ">=" => Some(Token::Operator(Operator::Relational(Comparison::MoreOrEqual))),
            "#" => Some(Token::Len),
            "repeat" => Some(Token::Repeat),
            "end" => Some(Token::End),
            "while" => Some(Token::While),
            "for" => Some(Token::For),
            "then" => Some(Token::Then),
            "until" => Some(Token::Until),
            "if" => Some(Token::If),
            "else" => Some(Token::Else),
            "elseif" => Some(Token::ElseIf),
            "return" => Some(Token::Return),
            "not" => Some(Token::Not),
            "do" => Some(Token::Do),
            "\"" | "\'" => Some(Token::Apostrophe),
            "true" => Some(Token::True),
            "false" => Some(Token::False),
            _ => { None }
        };
    }
}
