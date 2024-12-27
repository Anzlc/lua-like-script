use tokenizer::Token;
mod parser;
use parser::{ Parser, AstNode, ParsedValue, ForType };
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
            Token::Value(Value::Int(10)),
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
            Token::Value(Value::Int(5)),
            Token::Operator(tokenizer::Operator::Add),
            Token::Value(Value::Int(2)),
            Token::Operator(tokenizer::Operator::Multiply),
            Token::OpenParen,
            Token::Value(Value::Int(10)),
            Token::Operator(tokenizer::Operator::Add),
            Token::Value(Value::Int(2)),
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
        let ast = AstNode::Program(
            vec![AstNode::Assignment {
                is_local: false,
                variable: Box::new(AstNode::Variable("x".to_string())),
                rhs: Box::new(AstNode::BinaryOp {
                    op: tokenizer::Operator::Multiply,
                    lhs: Box::new(AstNode::BinaryOp {
                        op: tokenizer::Operator::Add,
                        lhs: Box::new(AstNode::Literal(ParsedValue::Int(10))),
                        rhs: Box::new(AstNode::FunctionCall {
                            name: Box::new(AstNode::Variable("y".to_string())),
                            args: vec![
                                AstNode::Literal(ParsedValue::Int(1)),
                                AstNode::Literal(ParsedValue::Int(10)),
                                AstNode::Literal(
                                    ParsedValue::String("\"Is this real chat\"".to_string())
                                )
                            ],
                        }),
                    }),
                    rhs: Box::new(AstNode::BinaryOp {
                        op: tokenizer::Operator::Power,
                        lhs: Box::new(AstNode::Literal(ParsedValue::Int(3))),
                        rhs: Box::new(AstNode::BinaryOp {
                            op: tokenizer::Operator::Power,
                            lhs: Box::new(AstNode::Literal(ParsedValue::Int(2))),
                            rhs: Box::new(AstNode::Literal(ParsedValue::Int(2))),
                        }),
                    }),
                }),
            }]
        );
        let mut parser = Parser::new(tokenizer.get_tokens().to_vec());
        let parsed = parser.parse();
        assert_eq!(parsed, ast);
    }

    #[test]
    fn local_assignment() {
        let code = "-- Simple code
        local x = 10
        ";
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());
        let ast = AstNode::Program(
            vec![AstNode::Assignment {
                is_local: true,
                variable: Box::new(AstNode::Variable("x".to_string())),
                rhs: Box::new(AstNode::Literal(ParsedValue::Int(10))),
            }]
        );
        let mut parser = Parser::new(tokenizer.get_tokens().to_vec());
        let parsed = parser.parse();
        assert_eq!(parsed, ast);
    }

    #[test]
    fn for_loop() {
        let code = "-- Simple code
        for i in 1, 10 do
            print(i)
        end
        ";
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());
        let ast = AstNode::Program(
            vec![AstNode::For {
                variable: "i".to_string(),
                for_type: ForType::Range {
                    start: Box::new(AstNode::Literal(ParsedValue::Int(1))),
                    end: Box::new(AstNode::Literal(ParsedValue::Int(10))),
                    step: Box::new(AstNode::Literal(ParsedValue::Int(1))),
                },
                scope: Box::new(AstNode::Scope {
                    stmts: vec![AstNode::FunctionCall {
                        name: Box::new(AstNode::Variable("print".to_string())),
                        args: vec![AstNode::Variable("i".to_string())],
                    }],
                }),
            }]
        );
        let mut parser = Parser::new(tokenizer.get_tokens().to_vec());
        let parsed = parser.parse();
        assert_eq!(parsed, ast);
    }

    #[test]
    fn while_loop() {
        let code = "-- Simple code
        while true do
            print(i)
        end
        ";
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());
        let ast = AstNode::Program(
            vec![AstNode::While {
                condition: Box::new(AstNode::Literal(ParsedValue::Bool(true))),
                scope: Box::new(AstNode::Scope {
                    stmts: vec![AstNode::FunctionCall {
                        name: Box::new(AstNode::Variable("print".to_string())),
                        args: vec![AstNode::Variable("i".to_string())],
                    }],
                }),
            }]
        );
        let mut parser = Parser::new(tokenizer.get_tokens().to_vec());
        let parsed = parser.parse();
        assert_eq!(parsed, ast);
    }

    #[test]
    fn repeat_until() {
        let code = "-- Simple code
        repeat
            print(i)
        until true
        ";
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());
        let ast = AstNode::Program(
            vec![AstNode::RepeatUntil {
                condition: Box::new(AstNode::Literal(ParsedValue::Bool(true))),
                scope: Box::new(AstNode::Scope {
                    stmts: vec![AstNode::FunctionCall {
                        name: Box::new(AstNode::Variable("print".to_string())),
                        args: vec![AstNode::Variable("i".to_string())],
                    }],
                }),
            }]
        );
        let mut parser = Parser::new(tokenizer.get_tokens().to_vec());
        let parsed = parser.parse();
        assert_eq!(parsed, ast);
    }

    #[test]
    fn if_statement() {
        let code = "-- Simple code
        if true then
            print(1)
        end
        ";
        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());
        let ast = AstNode::Program(
            vec![AstNode::If {
                condition: Box::new(AstNode::Literal(ParsedValue::Bool(true))),
                scope: Box::new(AstNode::Scope {
                    stmts: vec![AstNode::FunctionCall {
                        name: Box::new(AstNode::Variable("print".to_string())),
                        args: vec![AstNode::Literal(ParsedValue::Int(1))],
                    }],
                }),
                elseif: vec![],
                else_scope: Box::new(None),
            }]
        );
        let mut parser = Parser::new(tokenizer.get_tokens().to_vec());
        let parsed = parser.parse();
        assert_eq!(parsed, ast);
    }

    #[test]
    fn table_access() {
        let code =
            r#"
            -- Simple code
            local arr = {}
            arr[1] = 10
            arr[2] = 20
        "#;

        let mut tokenizer = Tokenizer::new();
        tokenizer.tokenize(code.to_string());

        let ast = AstNode::Program(
            vec![
                AstNode::Assignment {
                    is_local: true,
                    variable: AstNode::Variable("arr".to_string()).into(),
                    rhs: AstNode::Literal(ParsedValue::Table {
                        array: Vec::new(),
                        map: Vec::new(),
                    }).into(),
                },
                AstNode::Assignment {
                    is_local: false,
                    variable: (AstNode::Index {
                        base: AstNode::Variable("arr".to_string()).into(),
                        index: AstNode::Literal(ParsedValue::Int(1)).into(),
                    }).into(),
                    rhs: AstNode::Literal(ParsedValue::Int(10)).into(),
                },
                AstNode::Assignment {
                    is_local: false,
                    variable: (AstNode::Index {
                        base: AstNode::Variable("arr".to_string()).into(),
                        index: AstNode::Literal(ParsedValue::Int(2)).into(),
                    }).into(),
                    rhs: AstNode::Literal(ParsedValue::Int(20)).into(),
                }
            ]
        );

        let mut parser = Parser::new(tokenizer.get_tokens().to_vec());
        let parsed = parser.parse();

        assert_eq!(parsed, ast);
    }

    #[test]
    fn test1() {
        let code =
            "-- Simple code
            
        --local x = {1,2,3, [\"Hello\"]=10, name=10, name=\"10\", hello={10}}
        --print(x[1][1][1])
         x = (10 + y(1, 10,\"Is this real chat\")) * 3^2^2

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
