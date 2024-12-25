use tokenizer::Token;
mod parser;
use parser::{ Parser, AstNode };
mod tokenizer;

#[cfg(test)]
mod tests {
    use tokenizer::{ Tokenizer, Value };

    use super::*;

    #[test]
    fn tokenizer_assignment() {
        let code = "x = 10";
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());
        assert_eq!(tokenizer.get_tokens(), [
            Token::VariableOrFunction("x".to_string()),
            Token::Set,
            Token::Value(tokenizer::Value::Int(10)),
        ]);
    }
    #[test]
    fn tokenizer_comment() {
        let code = "-- Comment";
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());
        assert_eq!(tokenizer.get_tokens(), []);
    }
    #[test]
    fn tokenizer_block_comment() {
        let code =
            "--[[
        print(\"Random code\")
        let x = 5
         Comment
         --]]";
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());
        assert_eq!(tokenizer.get_tokens(), []);
    }
    #[test]
    fn tokenizer_equation() {
        let code = "5     +2*(   10+2)";
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());
        assert_eq!(tokenizer.get_tokens(), [
            Token::Value(tokenizer::Value::Int(5)),
            Token::Operator(tokenizer::Operator::Add),
            Token::Value(tokenizer::Value::Int(2)),
            Token::Operator(tokenizer::Operator::Multiply),
            Token::OpenParen,
            Token::Value(tokenizer::Value::Int(10)),
            Token::Operator(tokenizer::Operator::Add),
            Token::Value(tokenizer::Value::Int(2)),
            Token::CloseParen,
        ]);
    }

    #[test]
    fn parser_expression() {
        let code =
            "-- Simple code
        x = (10 + y(1, 10,\"Is this real chat\")) * 3^2^2
        
        ";
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());
        for t in tokenizer.get_tokens() {
            println!("{:?}", t);
        }
        let ast = AstNode::Program(
            [
                AstNode::Assignment {
                    is_local: false,
                    variable: "x".to_string(),
                    rhs: Box::new(AstNode::BinaryOp {
                        op: tokenizer::Operator::Multiply,
                        lhs: Box::new(AstNode::BinaryOp {
                            op: tokenizer::Operator::Add,
                            lhs: Box::new(AstNode::Literal(tokenizer::Value::Int(10))),
                            rhs: Box::new(AstNode::FunctionCall {
                                name: "y".to_string(),
                                args: [
                                    AstNode::Literal(tokenizer::Value::Int(1)),
                                    AstNode::Literal(tokenizer::Value::Int(10)),
                                    AstNode::Literal(
                                        tokenizer::Value::String(
                                            "\"Is this real chat\"".to_string()
                                        )
                                    ),
                                ].to_vec(),
                            }),
                        }),
                        rhs: Box::new(AstNode::BinaryOp {
                            op: tokenizer::Operator::Power,
                            lhs: Box::new(AstNode::Literal(tokenizer::Value::Int(3))),
                            rhs: Box::new(AstNode::BinaryOp {
                                op: tokenizer::Operator::Power,
                                lhs: Box::new(AstNode::Literal(tokenizer::Value::Int(2))),
                                rhs: Box::new(AstNode::Literal(tokenizer::Value::Int(2))),
                            }),
                        }),
                    }),
                },
            ].to_vec()
        );
        let mut parser = Parser::new(tokenizer.get_tokens().to_vec());
        let parsed = parser.parse();
        assert_eq!(parsed, ast)
    }

    #[test]
    fn local_assignment() {
        let code = "-- Simple code
        local x = 10
        
        ";
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());

        let ast = AstNode::Program(
            [
                AstNode::Assignment {
                    is_local: true,
                    variable: "x".to_string(),
                    rhs: Box::new(AstNode::Literal(Value::Int(10))),
                },
            ].to_vec()
        );
        let mut parser = Parser::new(tokenizer.get_tokens().to_vec());
        let parsed = parser.parse();
        assert_eq!(parsed, ast)
    }
    #[test]
    fn for_loop() {
        let code =
            "-- Simple code
        for i in 1,10 do
            print(i)
        end
        
        ";
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());

        let ast = AstNode::Program(
            [
                AstNode::For {
                    variable: "i".to_string(),
                    for_type: parser::ForType::Range {
                        start: Box::new(AstNode::Literal(Value::Int(1))),
                        end: Box::new(AstNode::Literal(Value::Int(10))),
                        step: Box::new(AstNode::Literal(Value::Int(1))),
                    },

                    scope: Box::new(AstNode::Scope {
                        stmts: [
                            AstNode::FunctionCall {
                                name: "print".to_string(),
                                args: [AstNode::Variable("i".to_string())].to_vec(),
                            },
                        ].to_vec(),
                    }),
                },
            ].to_vec()
        );
        let mut parser = Parser::new(tokenizer.get_tokens().to_vec());
        let parsed = parser.parse();
        assert_eq!(parsed, ast)
    }
    fn for_loop_generic() {
        let code =
            "-- Simple code
        for item in items do
            print(item)
        end
        
        ";
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());

        let ast = AstNode::Program(
            [
                AstNode::For {
                    variable: "item".to_string(),
                    for_type: parser::ForType::Generic(
                        Box::new(AstNode::Variable("items".to_string()))
                    ),
                    scope: Box::new(AstNode::Scope {
                        stmts: [
                            AstNode::FunctionCall {
                                name: "print".to_string(),
                                args: [AstNode::Variable("item".to_string())].to_vec(),
                            },
                        ].to_vec(),
                    }),
                },
            ].to_vec()
        );
        let mut parser = Parser::new(tokenizer.get_tokens().to_vec());
        let parsed = parser.parse();
        assert_eq!(parsed, ast)
    }
    fn while_loop() {
        let code =
            "-- Simple code
        while true do
            print(i)
        end
        
        ";
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());

        let ast = AstNode::Program(
            [
                AstNode::While {
                    condition: Box::new(AstNode::Literal(Value::Bool(true))),
                    scope: Box::new(AstNode::Scope {
                        stmts: [
                            AstNode::FunctionCall {
                                name: "print".to_string(),
                                args: [AstNode::Variable("i".to_string())].to_vec(),
                            },
                        ].to_vec(),
                    }),
                },
            ].to_vec()
        );
        let mut parser = Parser::new(tokenizer.get_tokens().to_vec());
        let parsed = parser.parse();
        assert_eq!(parsed, ast)
    }

    fn repeat_until() {
        let code =
            "-- Simple code
        repeat
            print(i)
        until true
        
        ";
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());

        let ast = AstNode::Program(
            [
                AstNode::RepeatUntil {
                    condition: Box::new(AstNode::Literal(Value::Bool(true))),
                    scope: Box::new(AstNode::Scope {
                        stmts: [
                            AstNode::FunctionCall {
                                name: "print".to_string(),
                                args: [AstNode::Variable("i".to_string())].to_vec(),
                            },
                        ].to_vec(),
                    }),
                },
            ].to_vec()
        );
        let mut parser = Parser::new(tokenizer.get_tokens().to_vec());
        let parsed = parser.parse();
        assert_eq!(parsed, ast)
    }

    #[test]
    fn if_statement() {
        #[test]
        fn parser_expression() {
            let code =
                "-- Simple code
            if true then
                print(1)
            end
            
            ";
            let mut tokenizer = Tokenizer::new();
            tokenizer.tokenize(code.to_string());
            for t in tokenizer.get_tokens() {
                println!("{:?}", t);
            }
            let ast = AstNode::Program(
                [
                    AstNode::If {
                        condition: Box::new(AstNode::Literal(Value::Bool(true))),
                        scope: Box::new(AstNode::Scope {
                            stmts: vec![AstNode::FunctionCall {
                                name: "print".to_string(),
                                args: vec![AstNode::Literal(Value::Int(1))],
                            }],
                        }),
                        elseif: vec![],
                        else_scope: Box::new(None),
                    },
                ].to_vec()
            );
            let mut parser = Parser::new(tokenizer.get_tokens().to_vec());
            let parsed = parser.parse();
            assert_eq!(parsed, ast)
        }
    }

    #[test]
    fn test1() {
        let code = "-- Simple code
            
        y += 10 + 1 * 2


        ";
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());
        for t in tokenizer.get_tokens() {
            println!("{:?}", t);
        }

        let mut parser = Parser::new(tokenizer.get_tokens().to_vec());
        let parsed = parser.parse();
        println!("{:#?}", parsed)
    }
}
