#[derive(Debug)]
pub enum LexerError {
    OperandFormatError,
    UnknownChar,
    LexError,
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperatorType {
    And,
    Or,
    Conditional,
    BiConditional,
    Non,
}

#[derive(Debug, PartialEq)]
pub enum BracketType {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Operand(bool),
    Operator(OperatorType),
    Bracket(BracketType),
}

/// A hand-written lexer
#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
    pos: usize,
}

struct Scanner {
    raw: String,
    pos: usize,
}

impl Scanner {
    fn new(s: &str) -> Scanner {
        Scanner {
            raw: String::from(s),
            pos: 0,
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.raw.chars().nth(self.pos)
    }

    fn next(&mut self) -> Option<char> {
        self.pos += 1;
        self.raw.chars().nth(self.pos - 1)
    }
}

impl Lexer {
    pub fn new(s: &str) -> Result<Lexer, LexerError> {
        let mut scanner = Scanner::new(s);
        let mut tokens = vec![];
        while let Some(ch) = scanner.peek() {
            match ch {
                // Operator
                '¬' | '∧' | '∨' | '→' | '↔' => {
                    let op = match ch {
                        '¬' => OperatorType::Non,
                        '∧' => OperatorType::And,
                        '∨' => OperatorType::Or,
                        '→' => OperatorType::Conditional,
                        '↔' => OperatorType::BiConditional,
                        _ => return Err(LexerError::LexError),
                    };
                    tokens.push(Token::Operator(op));
                    scanner.next();
                }
                // Operand
                'a'..='z' => {
                    let mut buf = String::new();
                    while let Some(ch) = scanner.peek() {
                        if !('a'..='z').contains(&ch) {
                            break;
                        }
                        buf.push(ch);
                        scanner.next();
                    }
                    tokens.push(Token::Operand(
                        buf.parse::<bool>()
                            .map_err(|_| LexerError::OperandFormatError)?,
                    ))
                }
                // Bracket
                '(' | ')' => {
                    let br = match ch {
                        '(' => BracketType::Left,
                        ')' => BracketType::Right,
                        _ => return Err(LexerError::LexError),
                    };
                    tokens.push(Token::Bracket(br));
                    scanner.next();
                }
                // Skip blank
                ' ' => {
                    scanner.next();
                }
                _ => return Err(LexerError::UnknownChar),
            }
        }
        Ok(Lexer { tokens, pos: 0 })
    }

    pub fn next(&mut self) -> Option<&Token> {
        self.pos += 1;
        self.tokens.get(self.pos - 1)
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("((true ∧ false) → true) ↔ false").unwrap();
        assert_eq!(lexer.next(), Some(&Token::Bracket(BracketType::Left)));
        assert_eq!(lexer.next(), Some(&Token::Bracket(BracketType::Left)));
        assert_eq!(lexer.next(), Some(&Token::Operand(true)));
        assert_eq!(lexer.next(), Some(&Token::Operator(OperatorType::And)));
        assert_eq!(lexer.next(), Some(&Token::Operand(false)));
        assert_eq!(lexer.next(), Some(&Token::Bracket(BracketType::Right)));
        assert_eq!(
            lexer.next(),
            Some(&Token::Operator(OperatorType::Conditional))
        );
        assert_eq!(lexer.next(), Some(&Token::Operand(true)));
        assert_eq!(lexer.next(), Some(&Token::Bracket(BracketType::Right)));
        assert_eq!(
            lexer.next(),
            Some(&Token::Operator(OperatorType::BiConditional))
        );
        assert_eq!(lexer.next(), Some(&Token::Operand(false)));
    }
}
