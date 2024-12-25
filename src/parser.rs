use std::{ os::windows::io::BorrowedSocket, thread::Scope };

use crate::tokenizer::{ Operator, Token, Value };

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Program(Vec<AstNode>),
    BinaryOp {
        op: Operator,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    Assignment {
        is_local: bool,
        variable: String,
        rhs: Box<AstNode>,
    },
    FunctionCall {
        name: String,
        args: Vec<AstNode>,
    },
    Variable(String),
    Literal(Value),
    Table {
        items: Vec<AstNode>,
    },
    UnaryOp {
        op: UnaryOp,
        value: Box<AstNode>,
    },
    Scope {
        stmts: Vec<AstNode>,
    },
    While {
        condition: Box<AstNode>,
        scope: Box<AstNode>,
    },
    If {
        condition: Box<AstNode>,
        scope: Box<AstNode>,
        elseif: Vec<AstNode>, // Contains AstNode::If with empty elseif and else scope
        else_scope: Box<Option<AstNode>>,
    },
    For {
        variable: String,
        for_type: ForType,
        scope: Box<AstNode>,
    },
    RepeatUntil {
        // A Do-While loop
        condition: Box<AstNode>,
        scope: Box<AstNode>,
    },
    FunctionDeclaration {
        name: String,
        arguments: Vec<String>,
        body: Box<AstNode>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForType {
    Generic(Box<AstNode>),
    Range {
        start: Box<AstNode>,
        end: Box<AstNode>,
        step: Box<AstNode>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negative,
    Length,
    Not,
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

        AstNode::Program(statements)
    }
    fn advance(&mut self) {
        self.index += 1;
    }
    fn parse_statement(&mut self) -> Option<AstNode> {
        return match self.get_current_token() {
            Some(Token::VariableOrFunction(_)) => { Some(self.parse_asignments_and_functions()) }
            Some(Token::Local) => { Some(self.parse_assignment()) }
            Some(Token::EndLine) => {
                self.advance();
                None
            }
            Some(Token::Do) => { Some(self.parse_do_end_scope()) }
            Some(Token::While) => { Some(self.parse_while()) }
            Some(Token::If) => { Some(self.parse_if()) }
            Some(Token::For) => { Some(self.parse_for()) }
            Some(Token::Repeat) => { Some(self.parse_repeat_until()) }
            Some(Token::Function) => { Some(self.parse_function()) }
            Some(t) => None,
            None => None,
        };
    }
    fn parse_repeat_until(&mut self) -> AstNode {
        if let Some(Token::Repeat) = self.get_current_token() {
            self.advance();
            let mut stmts = vec![];
            loop {
                if let Some(Token::Until) = self.get_current_token() {
                    self.advance();
                    break;
                }
                if let Some(v) = self.parse_statement() {
                    stmts.push(v);
                }
            }
            let expr = self.parse_expression();

            return AstNode::RepeatUntil {
                condition: Box::new(expr),
                scope: Box::new(AstNode::Scope { stmts: stmts }),
            };
        }
        unreachable!("Wrong repeat until syntax");
    }
    fn parse_for(&mut self) -> AstNode {
        /*
        for i in start,stop,step do
            <SCOPE>
        end

        or

        for i in some fucking shit do
        
        end
        */

        if let Some(Token::For) = self.get_current_token() {
            self.advance();
            if let Some(Token::VariableOrFunction(name)) = self.get_current_token() {
                let name = name.clone();
                self.advance();
                if let Some(Token::In) = self.get_current_token() {
                    self.advance();
                    let start = self.parse_expression();

                    if let Some(Token::Comma) = self.get_current_token() {
                        self.advance();
                        let end = self.parse_expression();
                        let mut step: AstNode = AstNode::Literal(Value::Int(1));
                        if let Some(Token::Comma) = self.get_current_token() {
                            self.advance();
                            step = self.parse_expression();
                        }
                        let for_type = ForType::Range {
                            start: Box::new(start),
                            end: Box::new(end),
                            step: Box::new(step),
                        };

                        let scope = self.parse_do_end_scope();
                        return AstNode::For {
                            variable: name.clone(),
                            for_type,
                            scope: Box::new(scope),
                        };
                    } else {
                        let expr = start; // It's not called start here
                        let for_type = ForType::Generic(Box::new(expr));

                        return AstNode::For {
                            variable: name.clone(),
                            for_type,
                            scope: Box::new(self.parse_do_end_scope()),
                        };
                    }
                }
            }
        }
        panic!("Wrong syntax for a for loop, are you dumb?")
    }

    fn parse_if(&mut self) -> AstNode {
        if let Some(Token::If) = self.get_current_token() {
            self.advance();
            let expr = self.parse_expression();
            if let Some(Token::Then) = self.get_current_token() {
                self.advance();
                let mut stmts = vec![];
                loop {
                    if let Some(Token::End) = self.get_current_token() {
                        self.advance();
                        let scope = AstNode::Scope { stmts: stmts };
                        return AstNode::If {
                            condition: Box::new(expr),
                            scope: Box::new(scope),
                            elseif: vec![],
                            else_scope: Box::new(None),
                        };
                    }
                    if let Some(Token::ElseIf) = self.get_current_token() {
                        let scope = AstNode::Scope { stmts: stmts };
                        let mut elseifs = vec![];
                        while let Some(elif_node) = self.parse_else_if_branches() {
                            println!("Elif NOde: {:#?}", elif_node);
                            elseifs.push(elif_node);
                        }
                        return AstNode::If {
                            condition: Box::new(expr),
                            scope: Box::new(scope),
                            elseif: elseifs,
                            else_scope: Box::new(self.parse_else_branch()),
                        };
                    }
                    if let Some(Token::Else) = self.get_current_token() {
                        let scope = AstNode::Scope { stmts: stmts };
                        return AstNode::If {
                            condition: Box::new(expr),
                            scope: Box::new(scope),
                            elseif: vec![],
                            else_scope: Box::new(self.parse_else_branch()),
                        };
                    }
                    if let Some(v) = self.parse_statement() {
                        stmts.push(v);
                    }
                }
            } else {
                panic!("Expected then after expression");
            }
        }
        unreachable!("NOnon")
    }
    fn parse_else_if_branches(&mut self) -> Option<AstNode> {
        if let Some(Token::ElseIf) = self.get_current_token() {
            println!("Ječp");
            self.advance();
            let expr = self.parse_expression();

            if let Some(Token::Then) = self.get_current_token() {
                let mut stmts = vec![];
                self.advance();
                loop {
                    println!("jkfkdloop");
                    println!("{:?}", self.get_current_token());
                    if let Some(Token::End) = self.get_current_token() {
                        self.advance();
                        let scope = AstNode::Scope { stmts: stmts };
                        println!("jeele");
                        return Some(AstNode::If {
                            condition: Box::new(expr),
                            scope: Box::new(scope),
                            elseif: vec![],
                            else_scope: Box::new(None),
                        });
                    }
                    if let Some(Token::ElseIf) = self.get_current_token() {
                        let scope = AstNode::Scope { stmts: stmts };
                        return Some(AstNode::If {
                            condition: Box::new(expr),
                            scope: Box::new(scope),
                            elseif: vec![],
                            else_scope: Box::new(None),
                        });
                    }
                    if let Some(Token::Else) = self.get_current_token() {
                        let scope = AstNode::Scope { stmts: stmts };
                        return Some(AstNode::If {
                            condition: Box::new(expr),
                            scope: Box::new(scope),
                            elseif: vec![],
                            else_scope: Box::new(None),
                        });
                    }

                    if let Some(v) = self.parse_statement() {
                        stmts.push(v);
                    }
                }
            } else {
                panic!("Expected then after expression");
            }
        }
        None
    }
    fn parse_else_branch(&mut self) -> Option<AstNode> {
        if let Some(Token::Else) = self.get_current_token() {
            self.advance();
            let mut stmts = vec![];
            loop {
                if let Some(Token::End) = self.get_current_token() {
                    self.advance();
                    let scope = AstNode::Scope { stmts: stmts };
                    return Some(scope);
                }
                if let Some(v) = self.parse_statement() {
                    stmts.push(v);
                }
            }
        }
        None
    }
    fn parse_while(&mut self) -> AstNode {
        if let Some(Token::While) = self.get_current_token() {
            self.advance();
            return AstNode::While {
                condition: Box::new(self.parse_expression()),
                scope: Box::new(self.parse_do_end_scope()),
            };
        }
        unreachable!("Nononoo");
    }
    fn parse_do_end_scope(&mut self) -> AstNode {
        if let Some(Token::Do) = self.get_current_token() {
            self.advance();
            let mut stmts = vec![];
            loop {
                if let Some(Token::End) = self.get_current_token() {
                    self.advance();
                    return AstNode::Scope { stmts: stmts };
                }
                if let Some(v) = self.parse_statement() {
                    stmts.push(v);
                }
            }
        }
        unreachable!("Should not reach if code is okie dokie")
    }
    fn parse_assignment(&mut self) -> AstNode {
        let is_local = match self.get_current_token() {
            Some(Token::Local) => {
                self.advance();
                true
            }
            _ => false,
        };

        match self.get_current_token() {
            Some(Token::VariableOrFunction(_)) => {}
            _ => panic!("Inccorect call to parse_asignments_and_functions "),
        }
        if let Some(Token::Set) = self.peek() {
            let variable = match self.get_current_token() {
                Some(Token::VariableOrFunction(name)) => {
                    let cloned = name.clone();
                    self.advance();
                    cloned
                }
                _ => panic!("Nonno"),
            };
            self.advance();
            return AstNode::Assignment {
                is_local: is_local,
                variable: variable,
                rhs: Box::new(self.parse_expression()),
            };
        }
        unreachable!("Incorrect assignment")
    }
    fn parse_asignments_and_functions(&mut self) -> AstNode {
        match self.get_current_token() {
            Some(Token::VariableOrFunction(_)) => {}
            _ => panic!("Inccorect call to parse_asignments_and_functions "),
        }
        if let Some(Token::OpenParen) = self.peek() {
            let variable = match self.get_current_token() {
                Some(Token::VariableOrFunction(name)) => {
                    let cloned = name.clone();
                    self.advance();
                    cloned
                }
                _ => panic!("Nonno"),
            };
            self.advance();
            let mut args: Vec<AstNode> = vec![];

            loop {
                if let Some(Token::CloseParen) = self.get_current_token() {
                    return AstNode::FunctionCall { name: variable, args: args };
                }
                args.push(self.parse_expression());

                if let Some(Token::Comma) = self.get_current_token() {
                    self.advance(); // Skip ,
                    continue;
                }

                self.advance();
                return AstNode::FunctionCall { name: variable, args: args };
            }
        }

        if let Some(Token::OperatorAssign(op)) = self.peek() {
            let op = op.clone();
            let variable = match self.get_current_token() {
                Some(Token::VariableOrFunction(name)) => {
                    let cloned = name.clone();
                    self.advance();
                    cloned
                }
                _ => panic!("Nonno"),
            };
            self.advance();
            let rhs = self.parse_expression();
            let var = AstNode::Variable(variable.clone());
            let expr = AstNode::BinaryOp { op: op, lhs: Box::new(var), rhs: Box::new(rhs) };

            return AstNode::Assignment { is_local: false, variable, rhs: Box::new(expr) };
        }

        return self.parse_assignment();
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

        if let Some(Token::Len) = self.get_current_token() {
            self.advance();
            let value = self.parse_factor();
            return AstNode::UnaryOp { op: UnaryOp::Length, value: Box::new(value) };
        }
        if let Some(Token::Not) = self.get_current_token() {
            self.advance();
            let value = self.parse_factor();
            return AstNode::UnaryOp { op: UnaryOp::Not, value: Box::new(value) };
        }
        if let Some(Token::OpenSquare) = self.get_current_token() {
            self.advance();
            //self.advance();
            let mut items: Vec<AstNode> = vec![];

            loop {
                println!("Self: {:?}", self.get_current_token());
                if let Some(Token::CloseSquare) = self.get_current_token() {
                    return AstNode::Table { items };
                }
                items.push(self.parse_expression());

                if let Some(Token::Comma) = self.get_current_token() {
                    self.advance(); // Skip ,
                    continue;
                }

                self.advance();
                return AstNode::Table { items };
            }
        }
        AstNode::Literal(Value::Nil)
    }

    fn parse_function(&mut self) -> AstNode {
        if let Some(Token::Function) = self.get_current_token() {
            println!("hello");
            self.advance();
            let name = match self.get_current_token() {
                Some(Token::VariableOrFunction(n)) => n.clone(),
                _ => panic!("Wrong syntax expected name"),
            };
            self.advance();
            if let Some(Token::OpenParen) = self.get_current_token() {
            } else {
                panic!("( not found");
            }
            self.advance();
            println!("{:?}", self.get_current_token());
            let mut args: Vec<String> = vec![];
            loop {
                if let Some(Token::CloseParen) = self.get_current_token() {
                    self.advance();
                    break;
                }
                if let Some(Token::VariableOrFunction(a)) = self.get_current_token() {
                    args.push(a.clone());
                    self.advance();
                } else {
                    self.advance(); // FIXME: Should check if comma
                }
            }
            let mut stmts: Vec<AstNode> = vec![];

            loop {
                if let Some(Token::End) = self.get_current_token() {
                    self.advance();
                    break;
                }
                if let Some(stmt) = self.parse_statement() {
                    stmts.push(stmt);
                }
            }

            return AstNode::FunctionDeclaration {
                name,
                arguments: args,
                body: Box::new(AstNode::Scope { stmts }),
            };
        }
        unreachable!("Nononon")
    }
}
