use fhe_ir::Op;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub enum Expr {
    Var(String),
    Const(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub struct Statement {
    pub var: String,
    pub expr: Expr,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String),
    UnexpectedEOF,
    InvalidNumber(String),
    ExpectedEquals,
    ExpectedIdentifier,
    ExpectedLet,
}

type ParseResult<T> = Result<T, ParseError>;

struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
    current: Option<char>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        let mut chars = input.chars().peekable();
        let current = chars.next();
        Parser { chars, current }
    }

    fn next_char(&mut self) -> Option<char> {
        self.current = self.chars.next();
        self.current
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current {
            if !c.is_whitespace() {
                break;
            }
            self.next_char();
        }
    }

    fn parse_number(&mut self) -> ParseResult<i64> {
        let mut num_str = String::new();
        
        while let Some(c) = self.current {
            if !c.is_digit(10) {
                break;
            }
            num_str.push(c);
            self.next_char();
        }

        num_str.parse::<i64>()
            .map_err(|_| ParseError::InvalidNumber(num_str))
    }

    fn parse_identifier(&mut self) -> ParseResult<String> {
        let mut ident = String::new();
        
        if let Some(c) = self.current {
            if !c.is_alphabetic() && c != '_' {
                return Err(ParseError::ExpectedIdentifier);
            }
            ident.push(c);
            self.next_char();
        }

        while let Some(c) = self.current {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            ident.push(c);
            self.next_char();
        }

        if ident.is_empty() {
            Err(ParseError::ExpectedIdentifier)
        } else {
            Ok(ident)
        }
    }

    fn parse_primary(&mut self) -> ParseResult<Expr> {
        self.skip_whitespace();
        
        match self.current {
            Some(c) if c.is_digit(10) => {
                let num = self.parse_number()?;
                Ok(Expr::Const(num))
            }
            Some(c) if c.is_alphabetic() || c == '_' => {
                let ident = self.parse_identifier()?;
                Ok(Expr::Var(ident))
            }
            Some('(') => {
                self.next_char();
                let expr = self.parse_expr()?;
                self.skip_whitespace();
                match self.current {
                    Some(')') => {
                        self.next_char();
                        Ok(expr)
                    }
                    _ => Err(ParseError::UnexpectedToken("Expected ')'".to_string())),
                }
            }
            Some(c) => Err(ParseError::UnexpectedToken(c.to_string())),
            None => Err(ParseError::UnexpectedEOF),
        }
    }

    fn parse_term(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_primary()?;

        loop {
            self.skip_whitespace();
            match self.current {
                Some('*') => {
                    self.next_char();
                    let right = self.parse_primary()?;
                    left = Expr::Mul(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_term()?;

        loop {
            self.skip_whitespace();
            match self.current {
                Some('+') => {
                    self.next_char();
                    let right = self.parse_term()?;
                    left = Expr::Add(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_statement(&mut self) -> ParseResult<Statement> {
        self.skip_whitespace();
        
        // Parse 'let'
        let ident = self.parse_identifier()?;
        if ident != "let" {
            return Err(ParseError::ExpectedLet);
        }

        self.skip_whitespace();
        
        // Parse variable name
        let var = self.parse_identifier()?;
        
        self.skip_whitespace();
        
        // Parse '='
        match self.current {
            Some('=') => {
                self.next_char();
            }
            _ => return Err(ParseError::ExpectedEquals),
        }

        // Parse expression
        let expr = self.parse_expr()?;

        // Skip optional semicolon
        self.skip_whitespace();
        if self.current == Some(';') {
            self.next_char();
        }

        Ok(Statement { var, expr })
    }
}

pub fn parse_dsl(input: &str) -> Result<Statement, ParseError> {
    let mut parser = Parser::new(input);
    parser.parse_statement()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_assignment() {
        let result = parse_dsl("let x = 42");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        assert_eq!(stmt.var, "x");
        match stmt.expr {
            Expr::Const(n) => assert_eq!(n, 42),
            _ => panic!("Expected Const expression"),
        }
    }

    #[test]
    fn test_addition() {
        let result = parse_dsl("let x = a + b");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        assert_eq!(stmt.var, "x");
        match stmt.expr {
            Expr::Add(a, b) => {
                match *a {
                    Expr::Var(ref name) => assert_eq!(name, "a"),
                    _ => panic!("Expected Var expression"),
                }
                match *b {
                    Expr::Var(ref name) => assert_eq!(name, "b"),
                    _ => panic!("Expected Var expression"),
                }
            }
            _ => panic!("Expected Add expression"),
        }
    }

    #[test]
    fn test_multiplication() {
        let result = parse_dsl("let x = a * b");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt.expr {
            Expr::Mul(_, _) => (),
            _ => panic!("Expected Mul expression"),
        }
    }

    #[test]
    fn test_complex_expression() {
        let result = parse_dsl("let result = x * y + 5");
        assert!(result.is_ok());
        let stmt = result.unwrap();
        match stmt.expr {
            Expr::Add(_, b) => {
                match *b {
                    Expr::Const(n) => assert_eq!(n, 5),
                    _ => panic!("Expected Const expression"),
                }
            }
            _ => panic!("Expected Add expression"),
        }
    }

    #[test]
    fn test_parentheses() {
        let result = parse_dsl("let x = (a + b) * c");
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_handling() {
        assert!(parse_dsl("x = 42").is_err()); // Missing 'let'
        assert!(parse_dsl("let x 42").is_err()); // Missing '='
        assert!(parse_dsl("let = 42").is_err()); // Missing variable name
        assert!(parse_dsl("let x = ").is_err()); // Missing expression
        assert!(parse_dsl("let x = )").is_err()); // Invalid expression
    }
}
