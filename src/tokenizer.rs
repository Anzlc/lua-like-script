use std::{ vec };

pub struct Tokenizer {
    tokens: Vec<Token>,
}

pub type MapEntry = (Value, Value);

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    String(String),
    Float(f64),
    Int(i64, usize),
    Bool(bool),
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
    Continue,
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
    BitwiseOr,
    BitwiseAnd,
    BitwiseXOR,
    BitwiseNot,
    BitwiseLShift,
    BitwiseRShift,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Comparison {
    Less,
    LessOrEqual,
    More,
    MoreOrEqual,
}

const SEPERATORS: &[&str] = &[" ", "\n", "\t", "\r"];
const OPERATORS: &[&str] = &[
    "&",
    "|",
    "!",
    "+",
    "-",
    "*",
    "/",
    "%",
    "^",
    "<",
    ">",
    "=",
    "~",
    ".",
    "#",
    ":",
];
const NON_EXTENDABLE: &[&str] = &[")", "(", ",", "[", "]", "{", "}"];
impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer { tokens: vec![] }
    }
    pub fn tokenize(&mut self, input: String) {
        let mut buf = String::new();

        let mut iterator = input.chars().peekable();
        'char_iter: while let Some(c) = iterator.next() {
            // TODO: Add seperate parsing for numbers
            //          Because float's get parsed wrong
            if c == '-' && *iterator.peek().unwrap_or(&'x') == '-' {
                iterator.next();
                self.add_token(Tokenizer::try_match_token(&buf));
                buf.clear();
                let x = iterator.next().unwrap_or('x');
                if x == '[' && *iterator.peek().unwrap_or(&'x') == '[' {
                    let mut line_count = 0;
                    iterator.next();
                    while let Some(c) = iterator.next() {
                        if c == '\n' {
                            line_count += 1;
                        }
                        if c == '-' && *iterator.peek().unwrap_or(&'x') == '-' {
                            iterator.next();
                            let x = iterator.next().unwrap_or('x');
                            if x == ']' && *iterator.peek().unwrap_or(&'x') == ']' {
                                iterator.next();

                                for _ in 0..line_count {
                                    self.add_token(Some(Token::EndLine));
                                }
                                continue 'char_iter;
                            }
                        }
                    }
                    // End of file
                    // Probably should throw error: Unmatched block comment
                    self.add_token(Some(Token::EndLine));
                    buf.clear();
                    continue 'char_iter; // Break should probably do the same
                } else {
                    for c in iterator.by_ref() {
                        if c == '\n' {
                            buf.clear();
                            self.add_token(Some(Token::EndLine));
                            continue 'char_iter;
                        }
                    }
                }
                self.add_token(Some(Token::EndLine));
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

                //x+=2
                loop {
                    let c = iterator.peek();
                    let mut new_buf = buf.clone();
                    if let Some(c) = c {
                        new_buf.push(*c);
                    }
                    if let Some(Token::Operator(_)) = Tokenizer::try_match_token(&new_buf) {
                        // Ok
                        buf = new_buf;
                        iterator.next();
                        continue;
                    }
                    if let Some(Token::OperatorAssign(_)) = Tokenizer::try_match_token(&new_buf) {
                        buf = new_buf;
                        iterator.next();
                        continue;
                    }

                    self.add_token(Tokenizer::try_match_token(&buf));
                    break;
                }

                eprintln!("Buf: {}", buf);
                buf.clear();

                continue;
            }
            if c == '\"' {
                self.add_token(Tokenizer::try_match_token(&buf));
                buf.clear();

                for c in iterator.by_ref() {
                    if c == '\"' {
                        self.add_token(Some(Token::Value(Value::String(buf.to_string()))));
                        buf.clear();
                        break;
                    }
                    buf.push(c);
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
        fn count_leading_zeros(input: &str) -> usize {
            input
                .chars()
                .take_while(|&c| c == '0')
                .count()
        }

        if let Some(t) = Tokenizer::get_token_from_keyword(token) {
            return Some(t);
        }
        if let Ok(t) = token.parse::<i64>() {
            println!("token: {}, count: {}", token, count_leading_zeros(token));
            return Some(Token::Value(Value::Int(t, count_leading_zeros(&token))));
        }
        if let Ok(t) = token.parse::<f64>() {
            return Some(Token::Value(Value::Float(t)));
        }

        if token.starts_with('"') && token.ends_with('"') {
            return Some(Token::Value(Value::String(String::from(token))));
        }
        if !token.is_empty() {
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
        match buf {
            "{" => Some(Token::OpenCurly),
            "}" => Some(Token::CloseCurly),
            "[" => Some(Token::OpenSquare),
            "]" => Some(Token::CloseSquare),
            "(" => Some(Token::OpenParen),
            ")" => Some(Token::CloseParen),
            "nil" => Some(Token::Value(Value::Nil)),
            "break" => Some(Token::Break),
            "continue" => Some(Token::Continue),
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
            "true" => Some(Token::Value(Value::Bool(true))),
            "false" => Some(Token::Value(Value::Bool(false))),
            "|" => Some(Token::Operator(Operator::BitwiseOr)),
            "&" => Some(Token::Operator(Operator::BitwiseAnd)),
            "~" => Some(Token::Operator(Operator::BitwiseNot)),
            "^^" => Some(Token::Operator(Operator::BitwiseXOR)),

            "<<" => Some(Token::Operator(Operator::BitwiseLShift)),
            ">>" => Some(Token::Operator(Operator::BitwiseRShift)),
            _ => { None }
        }
    }
}
