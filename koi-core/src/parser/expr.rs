use std::collections::HashMap;

use crate::ast::{BinaryOp, Expr, UnaryOp};
use crate::interp::Value;
use crate::token::{Token, TokenKind};

use super::Parser;

impl Parser {
    pub fn parse_expr(&mut self, min_bp: u8) -> Expr {
        let mut lhs = match self.lexer.next() {
            Some(Token { kind: TokenKind::Num(num), .. }) => Expr::Literal(Value::Num(num)),
            Some(Token { kind: TokenKind::Identifier(name), .. }) => Expr::Get(name),

            Some(t @ Token { kind: TokenKind::String { .. }, .. }) => self.continue_parse_string_expr(t),

            Some(Token { kind: TokenKind::True, .. }) => Expr::Literal(Value::Bool(true)),
            Some(Token { kind: TokenKind::False, .. }) => Expr::Literal(Value::Bool(false)),

            Some(Token { kind: TokenKind::Nil, .. }) => Expr::Literal(Value::Nil),

            Some(Token { kind: TokenKind::LeftBracket, .. }) => self.parse_vec_literal(),
            Some(Token { kind: TokenKind::LeftBrace, .. }) => self.parse_dict_literal(),

            Some(Token { kind: TokenKind::Fn, .. }) => self.parse_fn_lambda(),

            Some(t @ Token { .. }) if t.is_prefix_op() => {
                let kind = t.kind;
                let ((), r_bp) = prefix_binding_power(&kind).unwrap();

                self.lexer.consume_whitespace(self.is_multiline);
                let rhs = self.parse_expr(r_bp);

                make_prefix_expr(&kind, rhs)
            }

            Some(Token { kind: TokenKind::LeftParen, .. }) => {
                self.lexer.consume_whitespace(self.is_multiline);
                let expr = self.parse_expr(0);
                self.lexer.consume_whitespace(self.is_multiline);

                if !matches!(self.lexer.next(), Some(Token { kind: TokenKind::RightParen, .. })) {
                    panic!("expected right parenthesis");
                }

                expr
            }

            Some(Token { kind: TokenKind::DollarLeftParen, .. }) => {
                let cmd = self.parse_cmd(0);

                if !matches!(self.lexer.next(), Some(Token { kind: TokenKind::RightParen, .. })) {
                    panic!("expected right parenthesis");
                }

                Expr::Cmd(cmd)
            }

            _ => panic!("bad token"),
        };

        loop {
            self.lexer.consume_whitespace(self.is_multiline);

            if self.is_at_end() {
                break;
            }

            if matches!(self.lexer.peek(), Some(Token{kind: TokenKind::DotDot, ..})) {
                self.lexer.next().unwrap();

                self.lexer.consume_whitespace(self.is_multiline);
                let inclusive = if matches!(self.lexer.peek(), Some(Token{kind: TokenKind::Equal, ..})) {
                    self.lexer.next().unwrap();
                    true
                } else {
                    false
                };

                self.lexer.consume_whitespace(self.is_multiline);
                let rhs = self.parse_expr(0);

                return Expr::Range {
                    l: Box::new(lhs),
                    r: Box::new(rhs),
                    inclusive,
                };
            }

            let op = &self.lexer.peek().unwrap().kind;

            if let Some((l_bp, ())) = postfix_binding_power(op) {
                if l_bp < min_bp {
                    break;
                }

                let op = self.lexer.next().unwrap().kind;

                lhs = match op {
                    TokenKind::LeftBracket => {
                        self.lexer.consume_whitespace(self.is_multiline);
                        let index = self.parse_expr(0);
                        self.lexer.consume_whitespace(self.is_multiline);

                        if !matches!(self.lexer.next(), Some(Token { kind: TokenKind::RightBracket, .. })) {
                            panic!("expected right bracket");
                        }

                        Expr::GetField {
                            base: Box::new(lhs),
                            index: Box::new(index),
                        }
                    }
                    TokenKind::Dot => {
                        self.lexer.consume_whitespace(self.is_multiline);
                        let name = match self.lexer.next() {
                            Some(Token { kind: TokenKind::Identifier(name), .. }) => name,
                            _ => panic!("expected identifier"),
                        };

                        Expr::GetField {
                            base: Box::new(lhs),
                            index: Box::new(Expr::Literal(Value::String(name))),
                        }
                    }
                    TokenKind::LeftParen => self.parse_call(lhs),
                    _ => unreachable!(),
                };

                continue;
            }

            if let Some((l_bp, r_bp)) = infix_binding_power(&op) {
                if l_bp < min_bp {
                    break;
                }

                let op = self.lexer.next().unwrap().kind;

                self.lexer.consume_whitespace(self.is_multiline);
                let rhs = self.parse_expr(r_bp);

                lhs = make_infix_expr(lhs, &op, rhs);

                continue;
            }

            break;
        }

        lhs
    }

    fn consume_comma(&mut self) {
        self.lexer.consume_whitespace(self.is_multiline);
        if matches!(self.lexer.peek(), Some(Token{kind: TokenKind::Comma, ..})) {
            self.lexer.next();
        }
    }

    fn parse_call(&mut self, func: Expr) -> Expr {
        let func = Box::new(func);

        let mut args = Vec::new();

        loop {
            self.lexer.consume_whitespace(self.is_multiline);

            if matches!(self.lexer.peek(), Some(Token{kind: TokenKind::RightParen, ..})) {
                self.lexer.next();
                break;
            }

            args.push(self.parse_expr(0));

            self.consume_comma();
        }

        Expr::Call {
            args,
            func,
        }
    }

    fn parse_vec_literal(&mut self) -> Expr {
        let mut vec = Vec::new();

        loop {
            self.lexer.consume_whitespace(self.is_multiline);

            if matches!(self.lexer.peek(), Some(Token{kind: TokenKind::RightBracket, ..})) {
                self.lexer.next();
                break;
            }

            vec.push(self.parse_expr(0));

            self.consume_comma();
        }

        Expr::Vec(vec)
    }

    fn parse_dict_literal(&mut self) -> Expr {
        let mut dict = HashMap::new();

        loop {
            self.lexer.consume_whitespace(self.is_multiline);

            if matches!(self.lexer.peek(), Some(Token{kind: TokenKind::RightBrace, ..})) {
                self.lexer.next();
                break;
            }

            let k = match self.lexer.next() {
                Some(Token { kind: TokenKind::String { value, does_interp }, .. }) if !does_interp => value,
                Some(Token { kind: TokenKind::Identifier(name), .. }) => name,
                Some(Token { kind: TokenKind::Num(num), .. }) => num.to_string(),
                _ => panic!("bad dict key")
            };

            self.lexer.consume_whitespace(self.is_multiline);
            if !matches!(self.lexer.next(), Some(Token {kind: TokenKind::Colon, ..})) {
                panic!("expected colon");
            }

            self.lexer.consume_whitespace(self.is_multiline);
            let v = self.parse_expr(0);

            dict.insert(k, v);

            self.consume_comma();
        }

        Expr::Dict(dict)
    }

    fn parse_fn_lambda(&mut self) -> Expr {
        self.lexer.consume_whitespace(self.is_multiline);

        Expr::Lambda(self.continue_parse_fn())
    }

    pub fn continue_parse_string_expr(&mut self, t: Token) -> Expr {
        match t {
            Token { kind: TokenKind::String { value, does_interp: false }, .. } => {
                Expr::Literal(Value::String(value))
            }
            Token { kind: TokenKind::String { value, does_interp: true }, .. } => {
                let mut strings = Vec::new();
                let mut exprs = Vec::new();

                strings.push(value);

                loop {
                    self.lexer.consume_whitespace(self.is_multiline);
                    exprs.push(self.parse_expr(0));
                    self.lexer.consume_whitespace(self.is_multiline);

                    if let Some(Token { kind: TokenKind::String { value, does_interp }, .. }) = self.lexer.next() {
                        strings.push(value);

                        if !does_interp {
                            break;
                        }
                    } else {
                        panic!("bad token");
                    }
                }

                Expr::Interp { strings, exprs }
            }
            _ => panic!("bad token")
        }
    }
}

fn make_prefix_expr(op: &TokenKind, rhs: Expr) -> Expr {
    match *op {
        TokenKind::Plus => rhs,
        TokenKind::Minus => Expr::Unary(UnaryOp::Neg, Box::new(rhs)),
        TokenKind::Bang => Expr::Unary(UnaryOp::Not, Box::new(rhs)),

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
                Expr::GetField { base, index } => Expr::SetField { base, index, expr: rhs },
                _ => panic!("bad assignment target")
            }
        }

        _ => unreachable!(),
    }
}

fn prefix_binding_power(op: &TokenKind) -> Option<((), u8)> {
    use TokenKind::*;
    let bp = match op {
        Bang | Plus | Minus => ((), 17),
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

fn postfix_binding_power(op: &TokenKind) -> Option<(u8, ())> {
    use TokenKind::*;
    let bp = match op {
        LeftBracket | LeftParen | Dot => (19, ()),
        _ => return None,
    };
    Some(bp)
}

impl Token {
    fn is_prefix_op(&self) -> bool {
        prefix_binding_power(&self.kind).is_some()
    }
}
