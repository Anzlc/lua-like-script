use crate::tokenizer::{ Operator, Token, Value };

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Block(Vec<AstNode>),
    BinaryOp {
        op: Operator,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    Assignment {
        variable: String,
        rhs: Box<AstNode>,
    },
    FunctionCall {
        name: String,
        args: Vec<AstNode>,
    },
    Variable(String),
    Literal(Value),
    UnaryOp {
        op: UnaryOp,
        value: Box<AstNode>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negative, // Currently only negative
}

pub struct Parser {
    //TODO: Replace with linked list to allow popping at front
    tokens: Vec<Token>,
    index: usize,
}

#[derive(PartialEq, Eq)]
enum Associative {
    LEFT,
    RIGHT,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, index: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        return self.tokens.get(self.index + 1);
    }

    fn peek_at(&self, ahead: usize) -> Option<&Token> {
        return self.tokens.get(self.index + ahead);
    }

    fn get_current_token(&self) -> Option<&Token> {
        return self.tokens.get(self.index);
    }

    pub fn parse(&mut self) -> AstNode {
        let mut statements: Vec<AstNode> = vec![];

        while let Some(t) = self.get_current_token() {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
        }

        AstNode::Block(statements)
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn parse_statement(&mut self) -> Option<AstNode> {
        return match self.get_current_token() {
            Some(Token::VariableOrFunction(_)) => { Some(self.parse_assignment()) }
            Some(Token::EndLine) => {
                self.advance();
                None
            }
            Some(t) => unimplemented!("Joj res pa to še ne dela {:?}", t),
            None => panic!("Joj me ne"),
        };
    }
    fn parse_assignment(&mut self) -> AstNode {
        let variable = match self.get_current_token() {
            Some(Token::VariableOrFunction(name)) => {
                let cloned = name.clone();
                self.advance();
                cloned
            }
            _ => panic!("Nonno"),
        };

        if let Some(Token::Set) = self.get_current_token() {
            self.advance();
            return AstNode::Assignment {
                variable: variable,
                rhs: Box::new(self.parse_expression()),
            };
        } else {
            return AstNode::Literal(Value::Nil);
        }

        AstNode::Literal(Value::Nil)
    }
    fn parse_expression(&mut self) -> AstNode {
        // FIXME: Make it parse expression not just a number
        // println!("{:?}", self.get_current_token());
        // if let Some(Token::EndLine) = self.peek() {
        //     return self.parse_factor();
        // }
        // let mut node = self.parse_factor();

        // while let Some(Token::Operator(op)) = self.get_current_token() {
        //     let op = op.clone();
        //     self.advance();
        //     node = AstNode::BinaryOp {
        //         op,
        //         lhs: Box::new(node),
        //         rhs: Box::new(self.parse_factor()),
        //     };
        // }

        // return node;
        return self.parse_precedence_climbing(0);
    }
    fn parse_precedence_climbing(&mut self, min_prec: u8) -> AstNode {
        let mut res = self.parse_factor();
        let mut next_min_precedence;
        while let Some(Token::Operator(op)) = self.get_current_token() {
            let (prec, assoc) = Parser::get_precedence(op);

            if prec < min_prec {
                return res;
            }
            if assoc == Associative::LEFT {
                next_min_precedence = prec + 1;
            } else {
                next_min_precedence = prec;
            }
            let op = op.clone();
            self.advance();
            let rhs = self.parse_precedence_climbing(next_min_precedence);
            res = AstNode::BinaryOp { op: op.clone(), lhs: Box::new(res), rhs: Box::new(rhs) };
        }
        res
    }

    fn get_precedence(op: &Operator) -> (u8, Associative) {
        match op {
            Operator::Power => (6, Associative::RIGHT),
            Operator::Concatenation => (5, Associative::RIGHT),
            Operator::Multiply => (4, Associative::LEFT),
            Operator::Divide => (4, Associative::LEFT),
            Operator::FloorDivide => (4, Associative::LEFT),
            Operator::Mod => (4, Associative::LEFT),
            Operator::Subtract => (3, Associative::LEFT),
            Operator::Add => (3, Associative::LEFT),
            Operator::Relational(_) => (2, Associative::LEFT),
            Operator::Equals => (1, Associative::LEFT),
            Operator::NotEquals => (1, Associative::LEFT),
            Operator::And => (0, Associative::LEFT),
            Operator::Or => (0, Associative::LEFT),
        }
    }
    fn parse_factor(&mut self) -> AstNode {
        if let Some(t) = self.get_current_token() {
            if let Token::Value(v) = t {
                let v = v.clone();
                self.advance();
                return AstNode::Literal(v);
            }
        }
        if let Some(t) = self.get_current_token() {
            if let Token::VariableOrFunction(v) = t {
                let v = v.clone();
                if let Some(Token::OpenParen) = self.peek() {
                    self.advance();
                    self.advance();
                    let mut args: Vec<AstNode> = vec![];

                    loop {
                        println!("Self: {:?}", self.get_current_token());
                        if let Some(Token::CloseParen) = self.get_current_token() {
                            return AstNode::FunctionCall { name: v, args: args };
                        }
                        args.push(self.parse_expression());

                        if let Some(Token::Comma) = self.get_current_token() {
                            self.advance(); // Skip ,
                            continue;
                        }
                        println!("Selsssf: {:?}", self.get_current_token());
                        self.advance();
                        return AstNode::FunctionCall { name: v, args: args };
                    }
                }
                self.advance();
                return AstNode::Variable(v);
            }
        }
        if let Some(Token::OpenParen) = self.get_current_token() {
            self.advance();
            let expr = self.parse_expression();
            if let Some(Token::CloseParen) = self.get_current_token() {
                self.advance();
                return expr;
            }
            eprintln!("Unmatched (");
        }

        if let Some(Token::Operator(Operator::Subtract)) = self.get_current_token() {
            self.advance();
            println!("crr tokn: {:?}", self.get_current_token());

            let value = self.parse_factor();

            return AstNode::UnaryOp { op: UnaryOp::Negative, value: Box::new(value) };
        }
        AstNode::Literal(Value::Nil)
    }
}
