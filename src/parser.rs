use crate::errors::ParserError;
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
        target: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    FunctionCall {
        target: Box<AstNode>,
        args: Vec<AstNode>,
    },
    Variable(String),
    Literal(ParsedValue),

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
    Index {
        base: Box<AstNode>,
        index: Box<AstNode>,
    },
    Break,
    Continue,
    Return {
        expr: Box<AstNode>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParsedValue {
    Nil,
    String(String),
    Float(f64),
    Int(i64),
    Bool(bool),
    Table {
        array: Vec<AstNode>,
        map: Vec<(AstNode, AstNode)>,
    },
}
impl From<Value> for ParsedValue {
    fn from(value: Value) -> Self {
        match value {
            Value::Nil => ParsedValue::Nil,
            Value::Int(i, _) => ParsedValue::Int(i),
            Value::Float(f) => ParsedValue::Float(f),
            Value::String(s) => ParsedValue::String(s),
            Value::Bool(b) => ParsedValue::Bool(b),
        }
    }
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
    BitwiseNot,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TableEntry {
    Element(AstNode),
    KeyValue(AstNode, AstNode),
}

pub struct Parser {
    //TODO: Replace with linked list to allow popping at front
    tokens: Vec<Token>,
    index: usize,
    line_count: u32,
}

#[derive(PartialEq, Eq)]
enum Associative {
    LEFT,
    RIGHT,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, index: 0, line_count: 1 }
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
    pub fn parse(&mut self) -> Result<AstNode, ParserError> {
        let mut statements: Vec<AstNode> = vec![];

        while self.get_current_token().is_some() {
            match self.parse_statement() {
                Ok(stmt) => {
                    if let Some(stmt) = stmt {
                        statements.push(stmt);
                    }
                }
                Err(e) => {
                    return Err(ParserError::new(e, self.line_count));
                }
            }
        }

        Ok(AstNode::Program(statements))
    }
    fn advance(&mut self) {
        self.index += 1;
    }
    fn advance_token(&mut self, token: Token) -> Result<(), String> {
        if
            self.get_current_token().is_some() &&
            std::mem::discriminant(self.get_current_token().unwrap()) ==
                std::mem::discriminant(&token)
        {
            self.advance();
            return Ok(());
        }
        Err(format!("Expected {:?}, got {:?}", token, self.get_current_token()))
    }
    fn parse_statement(&mut self) -> Result<Option<AstNode>, String> {
        return match self.get_current_token() {
            Some(Token::VariableOrFunction(_)) => {
                Ok(Some(self.parse_asignments_and_functions()?))
            }
            Some(Token::Local) => { Ok(Some(self.parse_asignments_and_functions()?)) }
            Some(Token::EndLine) => {
                self.line_count += 1;
                self.advance();
                Ok(None)
            }
            Some(Token::Do) => { Ok(Some(self.parse_do_end_scope()?)) }
            Some(Token::While) => { Ok(Some(self.parse_while()?)) }
            Some(Token::If) => { Ok(Some(self.parse_if()?)) }
            Some(Token::For) => { Ok(Some(self.parse_for()?)) }
            Some(Token::Repeat) => { Ok(Some(self.parse_repeat_until()?)) }
            Some(Token::Function) => { Ok(Some(self.parse_function()?)) }
            Some(Token::Break) => {
                self.advance();
                return Ok(Some(AstNode::Break));
            }
            Some(Token::Continue) => {
                self.advance();
                return Ok(Some(AstNode::Continue));
            }
            Some(Token::Return) => Ok(Some(self.parse_return()?)),
            Some(t) => Err(format!("Unexpected token {:?}", t)),
            None => Ok(None),
        };
    }
    fn parse_repeat_until(&mut self) -> Result<AstNode, String> {
        if let Some(Token::Repeat) = self.get_current_token() {
            self.advance();
            let mut stmts = vec![];
            loop {
                if let Some(Token::Until) = self.get_current_token() {
                    self.advance();
                    break;
                }
                if let Some(v) = self.parse_statement()? {
                    stmts.push(v);
                }
            }
            let expr = self.parse_expression();

            return Ok(AstNode::RepeatUntil {
                condition: Box::new(expr?),
                scope: Box::new(AstNode::Scope { stmts }),
            });
        }
        Err("Invalid call to parse repeat until".to_string())
    }
    fn parse_for(&mut self) -> Result<AstNode, String> {
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
                        let mut step: AstNode = AstNode::Literal(ParsedValue::Int(1));
                        if let Some(Token::Comma) = self.get_current_token() {
                            self.advance();
                            step = self.parse_expression()?;
                        }
                        let for_type = ForType::Range {
                            start: Box::new(start?),
                            end: Box::new(end?),
                            step: Box::new(step),
                        };

                        let scope = self.parse_do_end_scope();
                        return Ok(AstNode::For {
                            variable: name.clone(),
                            for_type,
                            scope: Box::new(scope?),
                        });
                    } else {
                        let expr = start; // It's not called start here
                        let for_type = ForType::Generic(Box::new(expr?));

                        return Ok(AstNode::For {
                            variable: name.clone(),
                            for_type,
                            scope: Box::new(self.parse_do_end_scope()?),
                        });
                    }
                }
            }
        }
        Err("Invalid for loop syntax".to_string())
    }

    fn parse_if(&mut self) -> Result<AstNode, String> {
        if let Some(Token::If) = self.get_current_token() {
            self.advance();
            let expr = self.parse_expression();
            if let Some(Token::Then) = self.get_current_token() {
                self.advance();
                let mut stmts = vec![];
                loop {
                    if let Some(Token::End) = self.get_current_token() {
                        self.advance();
                        let scope = AstNode::Scope { stmts };
                        return Ok(AstNode::If {
                            condition: Box::new(expr?),
                            scope: Box::new(scope),
                            elseif: vec![],
                            else_scope: Box::new(None),
                        });
                    }
                    if let Some(Token::ElseIf) = self.get_current_token() {
                        let scope = AstNode::Scope { stmts };
                        let mut elseifs = vec![];
                        while let Some(elif_node) = self.parse_else_if_branches()? {
                            println!("Elif NOde: {:#?}", elif_node);
                            elseifs.push(elif_node);
                        }
                        return Ok(AstNode::If {
                            condition: Box::new(expr?),
                            scope: Box::new(scope),
                            elseif: elseifs,
                            else_scope: Box::new(self.parse_else_branch()?),
                        });
                    }
                    if let Some(Token::Else) = self.get_current_token() {
                        let scope = AstNode::Scope { stmts };
                        return Ok(AstNode::If {
                            condition: Box::new(expr?),
                            scope: Box::new(scope),
                            elseif: vec![],
                            else_scope: Box::new(self.parse_else_branch()?),
                        });
                    }
                    if let Some(v) = self.parse_statement()? {
                        println!("ASD{:?}", v);
                        stmts.push(v);
                    }
                }
            } else {
                return Err("Expected token then after expression".to_string());
            }
        }
        Err("Invalid call to parse if".to_string())
    }
    fn parse_else_if_branches(&mut self) -> Result<Option<AstNode>, String> {
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
                        let scope = AstNode::Scope { stmts };
                        println!("jeele");
                        return Ok(
                            Some(AstNode::If {
                                condition: Box::new(expr?),
                                scope: Box::new(scope),
                                elseif: vec![],
                                else_scope: Box::new(None),
                            })
                        );
                    }
                    if let Some(Token::ElseIf) = self.get_current_token() {
                        let scope = AstNode::Scope { stmts };
                        return Ok(
                            Some(AstNode::If {
                                condition: Box::new(expr?),
                                scope: Box::new(scope),
                                elseif: vec![],
                                else_scope: Box::new(None),
                            })
                        );
                    }
                    if let Some(Token::Else) = self.get_current_token() {
                        let scope = AstNode::Scope { stmts };
                        return Ok(
                            Some(AstNode::If {
                                condition: Box::new(expr?),
                                scope: Box::new(scope),
                                elseif: vec![],
                                else_scope: Box::new(None),
                            })
                        );
                    }

                    if let Some(v) = self.parse_statement()? {
                        stmts.push(v);
                    }
                }
            } else {
                return Err("Expected then after expression".to_string());
            }
        }
        Ok(None)
    }
    fn parse_else_branch(&mut self) -> Result<Option<AstNode>, String> {
        if let Some(Token::Else) = self.get_current_token() {
            self.advance();
            let mut stmts = vec![];
            loop {
                if let Some(Token::End) = self.get_current_token() {
                    self.advance();
                    let scope = AstNode::Scope { stmts };
                    return Ok(Some(scope));
                }
                if let Some(v) = self.parse_statement()? {
                    stmts.push(v);
                }
            }
        }
        Ok(None)
    }
    fn parse_while(&mut self) -> Result<AstNode, String> {
        if let Some(Token::While) = self.get_current_token() {
            self.advance();
            let expr = Box::new(self.parse_expression()?);
            println!("asd{:?}", expr);
            return Ok(AstNode::While {
                condition: expr,
                scope: Box::new(self.parse_do_end_scope()?),
            });
        }
        Err("Invalid call to parse while".to_string())
    }
    fn parse_do_end_scope(&mut self) -> Result<AstNode, String> {
        println!("asdasdasd{:?}", self.get_current_token());
        if let Some(Token::Do) = self.get_current_token() {
            self.advance();
            let mut stmts = vec![];
            loop {
                if let Some(Token::End) = self.get_current_token() {
                    self.advance();
                    return Ok(AstNode::Scope { stmts });
                }
                if let Some(v) = self.parse_statement()? {
                    stmts.push(v);
                }
            }
        }
        Err("Invalid call to parse do end".to_string())
    }

    fn parse_asignments_and_functions(&mut self) -> Result<AstNode, String> {
        let is_local = match self.get_current_token() {
            Some(Token::Local) => {
                self.advance();
                true
            }
            _ => false,
        };

        let target = match self.parse_target()? {
            Some(t) => t,
            None => {
                return Err("Target could not be parsed".to_string());
            }
        };

        if let Some(Token::OpenParen) = self.get_current_token() {
            self.advance();
            let mut args: Vec<AstNode> = vec![];

            loop {
                println!("{:?}", self.get_current_token());
                if let Some(Token::CloseParen) = self.get_current_token() {
                    self.advance_token(Token::CloseParen)?;
                    return Ok(AstNode::FunctionCall { target: Box::new(target), args });
                }
                args.push(self.parse_expression()?);

                if let Some(Token::Comma) = self.get_current_token() {
                    self.advance(); // Skip ,
                    continue;
                }

                self.advance_token(Token::CloseParen)?;
                return Ok(AstNode::FunctionCall { target: Box::new(target), args });
            }
        }
        if let Some(Token::OperatorAssign(op)) = self.get_current_token() {
            let op = op.clone();

            self.advance();
            let rhs = self.parse_expression();
            let expr = AstNode::BinaryOp {
                op,
                lhs: Box::new(target.clone()),
                rhs: Box::new(rhs?),
            };

            return Ok(AstNode::Assignment {
                is_local: false,
                target: Box::new(target),
                rhs: Box::new(expr),
            });
        }
        if let Some(Token::Set) = self.get_current_token() {
            self.advance();

            return Ok(AstNode::Assignment {
                is_local,
                target: Box::new(target),
                rhs: Box::new(self.parse_expression()?),
            });
        }

        Err("Could not parse assignment or function call".to_string())

        //return self.parse_assignment();
    }
    fn parse_expression(&mut self) -> Result<AstNode, String> {
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
        self.parse_precedence_climbing(0)
    }
    fn parse_precedence_climbing(&mut self, min_prec: u8) -> Result<AstNode, String> {
        let mut res = self.parse_factor()?;
        let mut next_min_precedence;
        while let Some(Token::Operator(op)) = self.get_current_token() {
            let (prec, assoc) = Parser::get_precedence(op);

            if prec < min_prec {
                return Ok(res);
            }
            if assoc == Associative::LEFT {
                next_min_precedence = prec + 1;
            } else {
                next_min_precedence = prec;
            }
            let op = op.clone();
            self.advance();
            let rhs = self.parse_precedence_climbing(next_min_precedence);
            res = AstNode::BinaryOp { op: op.clone(), lhs: Box::new(res), rhs: Box::new(rhs?) };
        }
        Ok(res)
    }
    fn get_precedence(op: &Operator) -> (u8, Associative) {
        match op {
            Operator::Power => (6, Associative::RIGHT),
            Operator::Concatenation => (5, Associative::RIGHT),
            Operator::BitwiseNot => (5, Associative::RIGHT), // Unary bitwise NOT
            Operator::Multiply => (4, Associative::LEFT),
            Operator::Divide => (4, Associative::LEFT),
            Operator::FloorDivide => (4, Associative::LEFT),
            Operator::Mod => (4, Associative::LEFT),
            Operator::BitwiseLShift => (3, Associative::LEFT), // Left shift
            Operator::BitwiseRShift => (3, Associative::LEFT), // Right shift
            Operator::Subtract => (3, Associative::LEFT),
            Operator::Add => (3, Associative::LEFT),
            Operator::BitwiseAnd => (2, Associative::LEFT), // Bitwise AND
            Operator::Relational(_) => (2, Associative::LEFT),
            Operator::BitwiseXOR => (1, Associative::LEFT), // Bitwise XOR
            Operator::Equals => (1, Associative::LEFT),
            Operator::NotEquals => (1, Associative::LEFT),
            Operator::BitwiseOr => (0, Associative::LEFT), // Bitwise OR
            Operator::And => (0, Associative::LEFT),
            Operator::Or => (0, Associative::LEFT),
        }
    }
    fn parse_factor(&mut self) -> Result<AstNode, String> {
        println!("current looking at: {:?}", self.get_current_token());

        println!("Targert???");
        if let Some(target) = self.parse_target()? {
            println!("Curur: {:?}", self.get_current_token());
            if let Some(Token::OpenParen) = self.get_current_token() {
                self.advance();

                let mut args: Vec<AstNode> = vec![];

                loop {
                    println!("Self: {:?}", self.get_current_token());
                    if let Some(Token::CloseParen) = self.get_current_token() {
                        self.advance();
                        return Ok(AstNode::FunctionCall { target: Box::new(target), args });
                    }
                    args.push(self.parse_expression()?);

                    if let Some(Token::Comma) = self.get_current_token() {
                        self.advance(); // Skip ,

                        continue;
                    }
                    println!("Selsssf: {:?}", self.get_current_token());
                    self.advance();
                    return Ok(AstNode::FunctionCall { target: Box::new(target), args });
                }
            }

            return Ok(target);
        }
        if let Some(Token::Value(v)) = self.get_current_token() {
            if let Value::Int(a, _) = v {
                if let Some(Token::Dot) = self.peek() {
                    if let Some(Token::Value(Value::Int(b, zeros))) = self.peek_at(2) {
                        let a = *a;
                        let b = *b;
                        let zeros = *zeros;

                        self.advance(); // Dot
                        self.advance(); //Number
                        self.advance(); // Other

                        if b == 0i64 {
                            return Ok(AstNode::Literal(ParsedValue::Float(a as f64)));
                        }
                        // Oh it's a float
                        println!(
                            "Not whole part: {}",
                            ((b as f64) /
                                (10.0f64).powi((b as f64).log10().ceil() as i32).max(10.0f64)) *
                                (10.0f64 * (zeros as f64))
                        );
                        let decimal_part =
                            (b as f64) /
                            (10.0f64).powi((b as f64).log10().ceil() as i32).max(10.0f64) /
                            (10.0f64).powi(zeros as i32);
                        let f = (a as f64) + decimal_part;

                        return Ok(AstNode::Literal(ParsedValue::Float(f)));
                    }
                }
            }
            println!("Hello");
            let v = v.clone();
            self.advance();
            return Ok(AstNode::Literal(v.into()));
        }
        println!("Hello sdss");

        if let Some(Token::OpenParen) = self.get_current_token() {
            self.advance();
            let expr = self.parse_expression();
            if let Some(Token::CloseParen) = self.get_current_token() {
                self.advance();
                return expr;
            }
            return Err("Expected )".to_string());
        }

        if let Some(Token::Operator(Operator::Subtract)) = self.get_current_token() {
            self.advance();
            println!("crr tokn: {:?}", self.get_current_token());
            println!("Okk");
            let value = self.parse_factor();
            println!("Okk 2 {:?}", value);
            return Ok(AstNode::UnaryOp { op: UnaryOp::Negative, value: Box::new(value?) });
        }
        if let Some(Token::Operator(Operator::BitwiseNot)) = self.get_current_token() {
            self.advance();
            println!("crr tokn: {:?}", self.get_current_token());

            let value = self.parse_factor();
            return Ok(AstNode::UnaryOp { op: UnaryOp::BitwiseNot, value: Box::new(value?) });
        }

        if let Some(Token::Len) = self.get_current_token() {
            self.advance();
            let value = self.parse_factor();
            return Ok(AstNode::UnaryOp { op: UnaryOp::Length, value: Box::new(value?) });
        }
        if let Some(Token::Not) = self.get_current_token() {
            self.advance();
            let value = self.parse_factor();
            return Ok(AstNode::UnaryOp { op: UnaryOp::Not, value: Box::new(value?) });
        }
        // if let Some(Token::OpenSquare) = self.get_current_token() {
        //     self.advance();
        //     //self.advance();
        //     let mut items: Vec<AstNode> = vec![];

        //     loop {
        //         println!("Self: {:?}", self.get_current_token());
        //         if let Some(Token::CloseSquare) = self.get_current_token() {
        //             return AstNode::Table { items };
        //         }
        //         items.push(self.parse_expression());

        //         if let Some(Token::Comma) = self.get_current_token() {
        //             self.advance(); // Skip ,
        //             continue;
        //         }

        //         self.advance();
        //         return AstNode::Table { items };
        //     }
        // }
        if let Some(Token::OpenCurly) = self.get_current_token() {
            return self.parse_table();
        }
        println!("Ended lookie");
        Err("Could not parse factor".to_string())
    }

    fn parse_table(&mut self) -> Result<AstNode, String> {
        if let Some(Token::OpenCurly) = self.get_current_token() {
            let mut elements: Vec<AstNode> = vec![];
            let mut map: Vec<(AstNode, AstNode)> = vec![];
            self.advance();
            loop {
                if let Some(Token::CloseCurly) = self.get_current_token() {
                    self.advance();
                    break;
                }
                match self.parse_table_entry()? {
                    TableEntry::Element(el) => elements.push(el),
                    TableEntry::KeyValue(key, value) => map.push((key, value)),
                }
                if let Some(Token::Comma) = self.get_current_token() {
                    self.advance(); // Skip ,
                    continue;
                }
            }
            let table = ParsedValue::Table { array: elements, map };
            return Ok(AstNode::Literal(table));
        }
        Err("Invalid table constructor".to_string())
    }

    fn parse_table_entry(&mut self) -> Result<TableEntry, String> {
        if let Some(Token::VariableOrFunction(name)) = self.get_current_token() {
            let name = name.clone();
            if let Some(Token::Set) = self.peek() {
                self.advance();
                self.advance();
                let expr = self.parse_expression();

                let entry = TableEntry::KeyValue(
                    AstNode::Literal(ParsedValue::String(name.clone())),
                    expr?
                );
                return Ok(entry);
            }
        }
        if let Some(Token::OpenSquare) = self.get_current_token() {
            self.advance();
            let expr = self.parse_expression();
            self.advance(); // Skip close ]
            if let Some(Token::Set) = self.get_current_token() {
                self.advance();
                let expr_value = self.parse_expression();
                return Ok(TableEntry::KeyValue(expr?, expr_value?));
            }
        }
        Ok(TableEntry::Element(self.parse_expression()?))
    }

    fn parse_function(&mut self) -> Result<AstNode, String> {
        if let Some(Token::Function) = self.get_current_token() {
            println!("hello");
            self.advance();
            let name = match self.get_current_token() {
                Some(Token::VariableOrFunction(n)) => n.clone(),
                _ => {
                    return Err("Invalid syntax for a function call expected name".to_string());
                }
            };
            self.advance();
            if let Some(Token::OpenParen) = self.get_current_token() {
            } else {
                return Err("Expected (".to_string());
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
                    self.advance_token(Token::Comma)?;
                }
            }
            let mut stmts: Vec<AstNode> = vec![];

            loop {
                if let Some(Token::End) = self.get_current_token() {
                    self.advance();
                    break;
                }
                if let Some(stmt) = self.parse_statement()? {
                    stmts.push(stmt);
                }
            }

            return Ok(AstNode::FunctionDeclaration {
                name,
                arguments: args,
                body: Box::new(AstNode::Scope { stmts }),
            });
        }
        unreachable!("Nononon")
    }

    fn parse_target(&mut self) -> Result<Option<AstNode>, String> {
        // FIXME: Fix this shit

        let mut base;

        if let Some(Token::VariableOrFunction(name)) = self.get_current_token() {
            base = AstNode::Variable(name.to_owned());
            self.advance();
        } else if let Some(Token::OpenCurly) = self.get_current_token() {
            base = self.parse_table()?;
        } else if let Some(Token::Value(Value::String(s))) = self.get_current_token() {
            println!("Let him cook");
            base = AstNode::Literal(ParsedValue::String(s.clone()));
            self.advance();
        } else {
            return Ok(None);
        }

        loop {
            match self.get_current_token() {
                Some(Token::Dot) => {
                    self.advance();
                    if let Some(Token::VariableOrFunction(i)) = self.get_current_token() {
                        let i = i.clone();
                        let index = AstNode::Literal(ParsedValue::String(i));
                        let indexed = AstNode::Index {
                            base: Box::new(base),
                            index: Box::new(index),
                        };
                        base = indexed;
                        self.advance();
                    } else {
                        return Err(
                            "Cannot index with a number value with . syntax. Try [] instead".to_string()
                        );
                    }
                }
                Some(Token::OpenSquare) => {
                    self.advance();
                    let expr = self.parse_expression()?;
                    if let Some(Token::CloseSquare) = self.get_current_token() {
                        let indexed = AstNode::Index {
                            base: Box::new(base),
                            index: Box::new(expr),
                        };
                        base = indexed;
                        self.advance();
                    }
                }
                _ => {
                    break;
                }
            }
        }
        return Ok(Some(base));
    }

    fn parse_return(&mut self) -> Result<AstNode, String> {
        if let Some(Token::Return) = self.get_current_token() {
            self.advance();
            return Ok(AstNode::Return { expr: Box::new(self.parse_expression()?) });
        }
        Err("Invalid call to parse return".to_string())
    }
}
