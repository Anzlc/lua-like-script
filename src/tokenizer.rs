use std::vec;

pub struct Tokenizer {
    tokens: Vec<Token>,
}

#[derive(Debug)]
pub enum Value {
    Nil,
    String(String),
    Number(f64),
    Bool(bool),
    List(Vec<Value>),
}

#[derive(Debug)]
pub enum Token {
    And,
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
    Or,
    Repeat,
    Return,
    Then,
    Until,
    While,
    OpPlus,
    OpMinus,
    OpMultiply,
    OpDivide,
    OpDivideFloorSet,
    OpMod,
    OpExponent,
    Len,
    Equals,
    NegEqual,
    LessOrEqual,
    GreaterOrEqual,
    Greater,
    Lower,
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
    DoubleDot,
    TripleDot,
    Apostrophe,
    VariableOrFunction(String),
    Value(Value),
}

const SEPERATORS: &'static [&str] = &[" ", "\n", "\t", "\r"];
const OPERATORS: &'static [&str] = &["+", "-", "*", "/", "%", "^", "<", ">", "="];

impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer { tokens: vec![] }
    }
    pub fn tokenize(&mut self, input: String) {
        let mut buf = String::new();

        let mut iterator = input.chars().peekable();
        while let Some(c) = iterator.next() {
            if SEPERATORS.contains(&c.to_string().as_str()) {
                self.add_token(Tokenizer::try_match_token(&buf));
                buf.clear();
                continue;
            }
            if OPERATORS.contains(&c.to_string().as_str()) {
                self.add_token(Tokenizer::try_match_token(&buf));
                buf.clear();
                buf.push(c);
                while let Some(c) = iterator.peek() {
                    if !OPERATORS.contains(&c.to_string().as_str()) {
                        self.add_token(Tokenizer::try_match_token(&buf));
                        buf.clear();

                        break;
                    }

                    if let Some(c) = iterator.next() {
                        buf.push(c);
                    }
                }
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

            buf.push(c);
        }
    }

    pub fn get_tokens(&self) -> &[Token] {
        &self.tokens
    }

    fn try_match_token(token: &str) -> Option<Token> {
        if let Some(t) = Tokenizer::get_token_from_keyword(token) {
            return Some(t);
        }
        if let Ok(t) = token.parse::<f64>() {
            return Some(Token::Value(Value::Number(t)));
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
            "and" => Some(Token::And),
            "or" => Some(Token::Or),
            "function" => Some(Token::Function),
            "local" => Some(Token::Local),
            "in" => Some(Token::In),
            "+" => Some(Token::OpPlus),
            "-" => Some(Token::OpMinus),
            "/" => Some(Token::OpDivide),
            "*" => Some(Token::OpMultiply),
            "%" => Some(Token::OpMod),
            "^" => Some(Token::OpExponent),
            ";" => Some(Token::Semicolon),
            "." => Some(Token::Dot),
            "," => Some(Token::Comma),
            ":" => Some(Token::Colon),
            ".." => Some(Token::DoubleDot),
            "..." => Some(Token::TripleDot),
            "=" => Some(Token::Set),
            "==" => Some(Token::Equals),
            "!=" => Some(Token::NegEqual),
            "<" => Some(Token::Lower),
            ">" => Some(Token::Greater),
            "<=" => Some(Token::LessOrEqual),
            ">=" => Some(Token::GreaterOrEqual),
            "#" => Some(Token::Len),
            "repeat" => Some(Token::Repeat),
            "while" => Some(Token::While),
            "for" => Some(Token::For),
            "then" => Some(Token::Then),
            "until" => Some(Token::Until),
            "if" => Some(Token::If),
            "else" => Some(Token::Else),
            "elseif" => Some(Token::ElseIf),
            "return" => Some(Token::ElseIf),
            "not" => Some(Token::ElseIf),
            "do" => Some(Token::ElseIf),
            "//=" => Some(Token::OpDivideFloorSet),
            "\"" | "\'" => Some(Token::Apostrophe),
            "true" => Some(Token::True),
            "false" => Some(Token::False),
            _ => { None }
        };
    }
}
