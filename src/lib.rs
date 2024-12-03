use tokenizer::Token;
use parser::Parser;
mod parser;
mod tokenizer;
use parser::AstNode;

#[cfg(test)]
mod tests {
    use tokenizer::Tokenizer;

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
        let ast = AstNode::Block(
            [
                AstNode::Assignment {
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
    fn test1() {
        let code =
            "-- Simple code
            --[[
        x = 10+-g(1)
        y = \"Hello\" .. \"World\"
        z = n + 1 < 10
        a = not 5==5
        b = #\"Hello world\"
        --]]
        do
            x = 1
            y = 10 + f(x) / 2
        end
        
        do
        end
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
