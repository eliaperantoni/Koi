use crate::scanner::Token;
use crate::ast::{Value, Expr, Stmt};

#[cfg(test)]
mod test;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
        }
    }

    /// Returns the next token without moving
    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn matches(&mut self, target: Token) -> bool {
        if self.peek() == target {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Returns the next token and moves to the next one
    fn advance(&mut self) -> Token {
        let token = self.peek();
        self.current += 1;
        token
    }

    /// Expects the next token to be equal to the provided one (panics otherwise) and moves to the
    /// next one
    fn consume(&mut self, token: Token) {
        if self.peek() != token {
            panic!("consume expected {:?} but got {:?}", token, self.peek());
        }
        self.advance();
    }

    /// Parses a series of statements (program)
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.peek() != Token::Eof {
            stmts.push(self.parse_stmt());
        }
        stmts
    }

    /// Parses a single statement
    fn parse_stmt(&mut self) -> Stmt {
        match self.peek() {
            Token::Var => self.parse_var_decl(),
            Token::If => self.parse_if(),
            _ => {
                let stmt = self.parse_expr(0).into();
                self.consume(Token::Semicolon);
                stmt
            }
        }
    }

    fn parse_if(&mut self) -> Stmt {
        unimplemented!();
    }

    fn parse_var_decl(&mut self) -> Stmt {
        // Consume Token::Var
        self.advance();

        let name = if let Token::Identifier { name } = self.advance() {
            name
        } else {
            panic!("expected identifier in var declaration");
        };

        let stmt = if self.matches(Token::Equal) {
            let expr = self.parse_expr(0);

            Stmt::Var {
                name,
                initializer: Some(expr),
            }
        } else {
            Stmt::Var {
                name,
                initializer: None,
            }
        };

        self.consume(Token::Semicolon);

        stmt
    }

    /// Parses an expression
    fn parse_expr(&mut self, min_bp: u8) -> Expr {
        use Token::*;

        let lhs = self.advance();
        let mut lhs = match lhs {
            Num { value } => Expr::Value(Value::Num(value)),
            String { mut value, mut does_interp, .. } => {
                if !does_interp {
                    return Expr::Value(Value::String(value.clone()));
                }

                let mut segments = Vec::new();
                let mut exprs = Vec::new();

                while does_interp {
                    segments.push(value.clone());

                    exprs.push(self.parse_expr(0));

                    if let String {
                        value: next_value,
                        does_interp: next_does_interp,
                        ..
                    } = self.advance() {
                        value = next_value;
                        does_interp = next_does_interp;
                    } else {
                        panic!("bad token, expected interpolation closing string");
                    }
                }

                segments.push(value.clone());

                Expr::Interp { segments, exprs }
            }
            True => Expr::Value(Value::Bool(true)),
            False => Expr::Value(Value::Bool(false)),
            LeftParen => {
                let lhs = self.parse_expr(0);
                assert_eq!(self.advance(), RightParen);
                lhs
            }
            Plus | Minus | PlusPlus | MinusMinus | Bang => {
                let ((), r_bp) = prefix_binding_power(&lhs);
                let rhs = self.parse_expr(r_bp);

                Expr::Unary {
                    rhs: Box::from(rhs),
                    op: lhs,
                }
            }
            t @ _ => panic!("bad token {:?}", t),
        };

        loop {
            let op = self.peek();

            if !(op.is_infix_op() || op == RightParen) {
                break;
            }

            if let Some((l_bp, r_bp)) = infix_binding_power(&op) {
                if l_bp < min_bp {
                    break;
                }

                self.advance();
                let rhs = self.parse_expr(r_bp);

                lhs = Expr::Binary {
                    lhs: Box::from(lhs),
                    rhs: Box::from(rhs),
                    op,
                };

                continue;
            }

            break;
        }

        lhs
    }
}

/// Returns the right binding power of a prefix operator
fn prefix_binding_power(op: &Token) -> ((), u8) {
    use Token::*;
    match op {
        Plus | Minus | PlusPlus | MinusMinus | Bang => ((), 15),
        _ => panic!("bad op {:?}", op),
    }
}

/// Returns the left and right binding power of an infix operator or None if the provided Token is
/// not an infix operator
fn infix_binding_power(op: &Token) -> Option<(u8, u8)> {
    use Token::*;
    let res = match op {
        Caret => (18, 17),
        Star | Slash | Perc => (13, 14),
        Plus | Minus => (11, 12),
        Less | LessEqual | Greater | GreaterEqual => (9, 10),
        EqualEqual | BangEqual => (7, 8),
        AmperAmper => (5, 6),
        PipePipe => (3, 4),
        Equal | PlusEqual | MinusEqual | StarEqual | SlashEqual | PercEqual | CaretEqual => (2, 1),
        _ => return None,
    };
    Some(res)
}
