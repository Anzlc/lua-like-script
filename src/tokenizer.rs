use std::vec;

pub struct Tokenizer;

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
    Value(Value),
}

const SEPERATORS: &'static str = " ";

impl Tokenizer {
    pub fn tokenize(input: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        let mut buf = String::new();

        let mut iterator = input.chars().peekable();
        while let Some(c) = iterator.next() {}

        tokens
    }
    fn get_token_from_keyword(buf: &str) -> Option<Token> {
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
            "\"" | "\'" => Some(Token::Apostrophe),
            "true" => Some(Token::True),
            "false" => Some(Token::False),
            _ => { None }
        };
    }
}
