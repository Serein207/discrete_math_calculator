use crate::lexer::{BracketType, Lexer, OperatorType, Token};

#[derive(Debug)]
pub enum ParseError {
    MissingBracket,
    Unmatch,
}

/// A recursive descent parser
pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser { lexer }
    }

    pub fn parse(&mut self) -> Result<Node, ParseError> {
        self.e()
    }

    fn e(&mut self) -> Result<Node, ParseError> {
        let t = self.t()?;
        self.equote(t)
    }

    fn equote(&mut self, val: Node) -> Result<Node, ParseError> {
        match self.lexer.peek() {
            Some(&Token::Operator(ref op @ (OperatorType::And | OperatorType::Or))) => {
                let op = op.clone();
                self.lexer.next();
                let t = self.t()?;
                self.equote(Node::BinaryExpr {
                    op,
                    lhs: Box::new(val),
                    rhs: Box::new(t),
                })
            }
            Some(&Token::Bracket(BracketType::Right)) | None => Ok(val),
            _ => Err(ParseError::Unmatch),
        }
    }

    fn t(&mut self) -> Result<Node, ParseError> {
        let t = self.v()?;
        self.tquote(t)
    }

    fn tquote(&mut self, val: Node) -> Result<Node, ParseError> {
        match self.lexer.peek() {
            Some(&Token::Operator(
                ref op @ (OperatorType::Conditional | OperatorType::BiConditional),
            )) => {
                let op = op.clone();
                self.lexer.next();
                let v = self.v()?;
                self.tquote(Node::BinaryExpr {
                    op,
                    lhs: Box::new(val),
                    rhs: Box::new(v),
                })
            }
            Some(
                &Token::Operator(OperatorType::And | OperatorType::Or)
                | &Token::Bracket(BracketType::Right),
            )
            | None => Ok(val),
            _ => Err(ParseError::Unmatch),
        }
    }

    fn v(&mut self) -> Result<Node, ParseError> {
        match self.lexer.peek() {
            Some(&Token::Operator(OperatorType::Non)) => {
                self.lexer.next();
                self.f(Some(OperatorType::Non))
            }
            _ => self.f(None),
        }
    }

    fn f(&mut self, val: Option<OperatorType>) -> Result<Node, ParseError> {
        match self.lexer.peek() {
            Some(&Token::Operand(n)) => {
                self.lexer.next();
                let node = Node::Operand(n);
                if let Some(OperatorType::Non) = val {
                    Ok(Node::UnaryExpr {
                        op: OperatorType::Non,
                        child: Box::new(node),
                    })
                } else {
                    Ok(node)
                }
            }
            Some(&Token::Bracket(BracketType::Left)) => {
                match self.lexer.next() {
                    Some(t) => t,
                    None => return Err(ParseError::Unmatch),
                };
                let e = self.e();
                match self.lexer.next() {
                    Some(&Token::Bracket(BracketType::Right)) => e,
                    _ => Err(ParseError::MissingBracket),
                }
            }
            Some(&Token::Operator(OperatorType::Non)) => {
                self.lexer.next();
                let child = self.f(None)?;
                Ok(Node::UnaryExpr {
                    op: OperatorType::Non,
                    child: Box::new(child),
                })
            }
            _ => return Err(ParseError::Unmatch),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Operand(bool),
    UnaryExpr {
        op: OperatorType,
        child: Box<Node>,
    },
    BinaryExpr {
        op: OperatorType,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parser() {
        let lexer = Lexer::new(" ((true ∧ false) → true) ↔ false").unwrap();
        let mut parser = Parser::new(lexer);
        let node = parser.parse();
        println!("{:?}", node);
        assert!(node.is_ok());
    }

    #[test]
    fn test_failed_parse() {
        let lexer = Lexer::new(" ((true ∧ false) → true) ↔").unwrap();
        let mut parser = Parser::new(lexer);
        let node = parser.parse();
        println!("{:?}", node);
        assert!(node.is_err());
    }

    #[test]
    fn test_unary() {
        let lexer = Lexer::new("(¬true ∧ false) → true").unwrap();
        let mut parser = Parser::new(lexer);
        let node = parser.parse();
        println!("{:?}", node);
        assert!(node.is_ok());
    }

    #[test]
    fn test_double_unary() {
        let lexer = Lexer::new("(¬¬true ∧ false) → true").unwrap();
        let mut parser = Parser::new(lexer);
        let node = parser.parse();
        println!("{:?}", node);
        assert!(node.is_ok());
    }
}
