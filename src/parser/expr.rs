use crate::token::{Token, TokenKind};
use crate::ast::{UnaryOp, BinaryOp, Value, Expr};
use super::Parser;
use std::collections::HashMap;

impl Parser {
    pub fn parse_expression(&mut self, min_bp: u8) -> Result<Expr, &'static str> {
        let mut lhs = match self.lexer.next() {
            Some(Token { kind: TokenKind::Num(num), .. }) => Expr::Literal(Value::Num(num)),
            Some(Token { kind: TokenKind::Identifier(name), .. }) => Expr::Get(name),

            Some(Token { kind: TokenKind::String { value, does_interp: false }, .. }) => Expr::Literal(Value::String(value)),
            Some(Token { kind: TokenKind::String { value, does_interp: true }, .. }) => {
                let mut strings = Vec::new();
                let mut exprs = Vec::new();

                strings.push(value);

                loop {
                    exprs.push(self.parse_expression(0)?);

                    if let Some(Token { kind: TokenKind::String { value, does_interp }, .. }) = self.lexer.next() {
                        strings.push(value);

                        if !does_interp {
                            break;
                        }
                    } else {
                        return Err("bad token");
                    }
                }

                Expr::Interp { strings, exprs }
            }

            Some(Token { kind: TokenKind::True, .. }) => Expr::Literal(Value::Bool(true)),
            Some(Token { kind: TokenKind::False, .. }) => Expr::Literal(Value::Bool(false)),

            Some(Token { kind: TokenKind::Nil, .. }) => Expr::Literal(Value::Nil),

            Some(Token { kind: TokenKind::LeftBracket, .. }) => self.parse_vec_literal()?,
            Some(Token { kind: TokenKind::LeftBrace, .. }) => self.parse_dict_literal()?,

            Some(Token { kind, .. }) if prefix_binding_power(&kind).is_some() => {
                let ((), r_bp) = prefix_binding_power(&kind).unwrap();
                let rhs = self.parse_expression(r_bp)?;

                make_prefix_expr(&kind, rhs)
            }

            Some(Token { kind: TokenKind::LeftParen, .. }) => {
                let expr = self.parse_expression(0)?;
                if !matches!(self.lexer.next(), Some(Token { kind: TokenKind::RightParen, .. })) {
                    return Err("expected right parenthesis")
                }
                expr
            }

            _ => return Err("bad token"),
        };

        loop {
            let op = match self.lexer.peek() {
                Some(Token { kind, .. }) => kind,
                None => break,
            };

            if let Some((l_bp, ())) = postfix_binding_power(op) {
                if l_bp < min_bp {
                    break;
                }

                let op = self.lexer.next().unwrap().kind;

                lhs = match op {
                    TokenKind::LeftBracket => {
                        let index = self.parse_expression(0)?;

                        if !matches!(self.lexer.next(), Some(Token { kind: TokenKind::RightBracket, .. })) {
                            return Err("expected right bracket");
                        }

                        Expr::GetField {
                            base: Box::new(lhs),
                            index: Box::new(index),
                        }
                    }
                    TokenKind::Dot => {
                        let name = match self.lexer.next() {
                            Some(Token { kind: TokenKind::Identifier(name), .. }) => name,
                            _ => return Err("expected identifier"),
                        };

                        Expr::GetField {
                            base: Box::new(lhs),
                            index: Box::new(Expr::Literal(Value::String(name))),
                        }
                    }
                    TokenKind::LeftParen => self.parse_call(lhs)?,
                    _ => make_postfix_expr(lhs, &op),
                };
                continue;
            }

            if let Some((l_bp, r_bp)) = infix_binding_power(&op) {
                if l_bp < min_bp {
                    break;
                }

                let op = self.lexer.next().unwrap().kind;

                let rhs = self.parse_expression(r_bp)?;
                lhs = make_infix_expr(lhs, &op, rhs);
                continue;
            }

            break;
        }

        Ok(lhs)
    }

    fn parse_call(&mut self, func: Expr) -> Result<Expr, &'static str> {
        let mut args = Vec::new();

        match self.lexer.peek() {
            Some(Token { kind: TokenKind::RightParen, .. }) => {
                self.lexer.next();
                return Ok(Expr::Call {
                    args,
                    func: Box::new(func),
                });
            }
            _ => ()
        }

        loop {
            args.push(self.parse_expression(0)?);

            match self.lexer.next() {
                Some(Token { kind: TokenKind::Comma, .. }) => (),
                Some(Token { kind: TokenKind::RightParen, .. }) => break,
                _ => return Err("expected comma or right parenthesis"),
            }
        }

        Ok(Expr::Call {
            args,
            func: Box::new(func),
        })
    }

    fn parse_vec_literal(&mut self) -> Result<Expr, &'static str> {
        let mut vec = Vec::new();

        match self.lexer.peek() {
            Some(Token { kind: TokenKind::RightBracket, .. }) => {
                self.lexer.next();
                return Ok(Expr::Vec(vec));
            }
            _ => ()
        }

        loop {
            vec.push(self.parse_expression(0)?);

            match self.lexer.next() {
                Some(Token { kind: TokenKind::Comma, .. }) => (),
                Some(Token { kind: TokenKind::RightBracket, .. }) => break,
                _ => return Err("expected comma or right bracket"),
            }
        }

        Ok(Expr::Vec(vec))
    }

    fn parse_dict_literal(&mut self) -> Result<Expr, &'static str> {
        let mut dict = HashMap::new();

        match self.lexer.peek() {
            Some(Token { kind: TokenKind::RightBrace, .. }) => {
                self.lexer.next();
                return Ok(Expr::Dict(dict));
            }
            _ => ()
        }

        loop {
            let k = match self.lexer.next() {
                Some(Token { kind: TokenKind::String { value, does_interp }, .. }) if !does_interp => value,
                Some(Token { kind: TokenKind::Identifier(name), .. }) => name,
                Some(Token { kind: TokenKind::Num(num), .. }) => num.to_string(),
                _ => return Err("bad dict key")
            };

            if !matches!(self.lexer.next(), Some(Token {kind: TokenKind::Colon, ..})) {
                return Err("expected colon");
            }

            let v = self.parse_expression(0)?;

            dict.insert(k, v);

            match self.lexer.next() {
                Some(Token { kind: TokenKind::Comma, .. }) => (),
                Some(Token { kind: TokenKind::RightBrace, .. }) => break,
                _ => return Err("expected comma or right brace"),
            }
        }

        Ok(Expr::Dict(dict))
    }
}

fn make_postfix_expr(lhs: Expr, op: &TokenKind) -> Expr {
    match *op {
        TokenKind::PlusPlus => Expr::Unary(UnaryOp::PostInc, Box::new(lhs)),
        TokenKind::MinusMinus => Expr::Unary(UnaryOp::PostDec, Box::new(lhs)),
        _ => unreachable!()
    }
}

fn make_prefix_expr(op: &TokenKind, rhs: Expr) -> Expr {
    match *op {
        TokenKind::Plus => rhs,
        TokenKind::Minus => Expr::Unary(UnaryOp::Neg, Box::new(rhs)),
        TokenKind::Bang => Expr::Unary(UnaryOp::Not, Box::new(rhs)),

        TokenKind::PlusPlus => Expr::Unary(UnaryOp::PreInc, Box::new(rhs)),
        TokenKind::MinusMinus => Expr::Unary(UnaryOp::PreDec, Box::new(rhs)),

        _ => unreachable!()
    }
}

fn make_infix_expr(lhs: Expr, op: &TokenKind, rhs: Expr) -> Expr {
    let lhs = Box::new(lhs);
    let rhs = Box::new(rhs);

    match *op {
        TokenKind::Plus => Expr::Binary(lhs, BinaryOp::Sum, rhs),
        TokenKind::Minus => Expr::Binary(lhs, BinaryOp::Sub, rhs),
        TokenKind::Star => Expr::Binary(lhs, BinaryOp::Mul, rhs),
        TokenKind::Slash => Expr::Binary(lhs, BinaryOp::Div, rhs),
        TokenKind::Perc => Expr::Binary(lhs, BinaryOp::Mod, rhs),
        TokenKind::Caret => Expr::Binary(lhs, BinaryOp::Pow, rhs),

        TokenKind::AmperAmper => Expr::Binary(lhs, BinaryOp::And, rhs),
        TokenKind::PipePipe => Expr::Binary(lhs, BinaryOp::Or, rhs),

        TokenKind::EqualEqual | TokenKind::BangEqual => {
            let mut expr = Expr::Binary(lhs, BinaryOp::Equal, rhs);

            if matches!(op, TokenKind::BangEqual) {
                expr = Expr::Unary(UnaryOp::Not, Box::new(expr));
            }

            expr
        }

        TokenKind::Great | TokenKind::GreatEqual | TokenKind::Less | TokenKind::LessEqual => {
            let mut expr = Expr::Binary(
                lhs.clone(),
                match op {
                    TokenKind::Great | TokenKind::GreatEqual => BinaryOp::Great,
                    TokenKind::Less | TokenKind::LessEqual => BinaryOp::Less,
                    _ => unreachable!(),
                },
                rhs.clone(),
            );

            if matches!(op, TokenKind::GreatEqual | TokenKind::LessEqual) {
                let lhs = lhs.clone();
                let rhs = rhs.clone();

                expr = Expr::Binary(
                    Box::new(expr),
                    BinaryOp::Or,
                    Box::new(Expr::Binary(lhs, BinaryOp::Equal, rhs)),
                );
            }

            expr
        }

        TokenKind::Equal |
        TokenKind::PlusEqual | TokenKind::MinusEqual |
        TokenKind::StarEqual | TokenKind::SlashEqual |
        TokenKind::PercEqual | TokenKind::CaretEqual => {
            let rhs = if !matches!(*op, TokenKind::Equal) {
                let op = match *op {
                    TokenKind::PlusEqual => BinaryOp::Sum,
                    TokenKind::MinusEqual => BinaryOp::Sub,
                    TokenKind::StarEqual => BinaryOp::Mul,
                    TokenKind::SlashEqual => BinaryOp::Div,
                    TokenKind::PercEqual => BinaryOp::Mod,
                    TokenKind::CaretEqual => BinaryOp::Pow,
                    _ => unreachable!()
                };

                Box::new(Expr::Binary(lhs.clone(), op, rhs))
            } else {
                rhs
            };

            match *lhs {
                Expr::Get(name) => Expr::Set(name, rhs),
                Expr::GetField { base, index } => Expr::SetField { base, index, value: rhs },
                _ => panic!("bad assignment target")
            }
        }

        _ => unreachable!(),
    }
}

fn postfix_binding_power(op: &TokenKind) -> Option<(u8, ())> {
    use TokenKind::*;
    let bp = match op {
        LeftBracket | LeftParen | Dot => (21, ()),
        PlusPlus | MinusMinus => (19, ()),
        _ => return None,
    };
    Some(bp)
}

fn prefix_binding_power(op: &TokenKind) -> Option<((), u8)> {
    use TokenKind::*;
    let bp = match op {
        Plus | Minus | PlusPlus | MinusMinus => ((), 17),
        _ => return None,
    };
    Some(bp)
}

fn infix_binding_power(op: &TokenKind) -> Option<(u8, u8)> {
    use TokenKind::*;
    let bp = match op {
        Caret => (16, 15),
        Star | Slash | Perc => (13, 14),
        Plus | Minus => (11, 12),
        Great | GreatEqual | Less | LessEqual => (9, 10),
        EqualEqual | BangEqual => (7, 8),
        AmperAmper => (5, 6),
        PipePipe => (3, 4),
        Equal | PlusEqual | MinusEqual | StarEqual | SlashEqual | PercEqual | CaretEqual => (2, 1),
        _ => return None,
    };
    Some(bp)
}
