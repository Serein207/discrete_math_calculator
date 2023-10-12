use crate::lexer::{Lexer, OperatorType};
use crate::parser::{Node, Parser};

pub struct Calculator {
    ast: Node,
}

impl Calculator {
    pub fn build(expr: &str) -> Self {
        Calculator {
            ast: Parser::new(Lexer::new(expr).expect("Failed to lex"))
                .parse()
                .expect("Failed to parse"),
        }
    }

    pub fn eval(&mut self) -> bool {
        Self::eval_helper(&self.ast)
    }

    fn eval_helper(node: &Node) -> bool {
        match node {
            Node::Operand(n) => *n,
            Node::UnaryExpr { op, child } => {
                let child = Self::eval_helper(child);
                match op {
                    OperatorType::Non => !child,
                    _ => child,
                }
            }
            Node::BinaryExpr { op, lhs, rhs } => {
                let lhs_ret = Self::eval_helper(lhs);
                let rhs_ret = Self::eval_helper(rhs);

                match op {
                    OperatorType::And => lhs_ret && rhs_ret,
                    OperatorType::Or => lhs_ret || rhs_ret,
                    OperatorType::Conditional => (!lhs_ret) || rhs_ret,
                    OperatorType::BiConditional => lhs_ret == rhs_ret,
                    _ => false,
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_calculator() {
        let result = Calculator::build("((true ∧ false) → true) ↔ true").eval();
        assert!(result);
    }
}
