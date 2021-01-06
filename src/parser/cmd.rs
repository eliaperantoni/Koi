use crate::ast::{Cmd, CmdOp, Expr, Value};
use crate::token::{Token, TokenKind};

use super::Parser;

impl Parser {
    pub fn parse_cmd(&mut self, min_bp: u8) -> Cmd {
        let mut lhs = {
            let mut segments = Vec::new();

            loop {
                let mut exprs = Vec::new();

                loop {
                    if self.lexer.peek().is_none() {
                        break;
                    }

                    if {
                        let t = self.lexer.peek().unwrap();
                        binding_power(&t.kind).is_some() || [TokenKind::Space, TokenKind::Newline].contains(&t.kind)
                    } {
                        break;
                    }

                    let expr = match self.lexer.next().unwrap() {
                        t @ Token { kind: TokenKind::String { .. }, .. } => self.continue_parse_string_expr(t),
                        Token { kind: TokenKind::LeftBrace, .. } => {
                            self.lexer.consume_whitespace();
                            let expr = self.parse_expression(0);
                            self.lexer.consume_whitespace();

                            if !matches!(self.lexer.next(), Some(Token { kind: TokenKind::RightBrace, .. })) {
                                panic!("expected right brace");
                            }

                            Expr::Interp {
                                exprs: vec![expr],
                                strings: vec!["".to_owned(), "".to_owned()],
                            }
                        }
                        t => Expr::Literal(Value::String(t.lexeme)),
                    };

                    exprs.push(expr);
                }

                segments.push(exprs);


                if self.lexer.peek().unwrap().kind == TokenKind::Space {
                    self.lexer.next();
                } else {
                    break;
                }
            }

            Cmd::Atom(segments)
        };

        loop {
            let op = match self.lexer.peek() {
                Some(Token { kind, .. }) if kind != &TokenKind::Newline => kind,

                // Should only match if none or newline
                _ => break
            };

            // Should be safe to do because previous step consume all tokens that are not valid ops
            let (l_bp, r_bp) = binding_power(op).unwrap();
            if l_bp < min_bp {
                break;
            }

            self.lexer.next();
            let rhs = self.parse_cmd(r_bp);

            lhs = Cmd::Op(
                Box::new(lhs),
                match op {
                    TokenKind::PipePipe => CmdOp::And,
                    TokenKind::AmperAmper => CmdOp::Or,

                    TokenKind::Pipe => CmdOp::OutPipe,
                    TokenKind::StarPipe => CmdOp::ErrPipe,
                    TokenKind::AmperPipe => CmdOp::AllPipe,

                    TokenKind::Great => CmdOp::OutWrite,
                    TokenKind::StarGreat => CmdOp::ErrWrite,
                    TokenKind::AmperGreat => CmdOp::AllWrite,

                    TokenKind::Less => CmdOp::OutRead,
                    TokenKind::StarLess => CmdOp::ErrRead,
                    TokenKind::AmperLess => CmdOp::AllRead,

                    _ => unreachable!()
                },
                Box::new(rhs),
            );
        }

        lhs
    }
}

fn binding_power(op: &TokenKind) -> Option<(u8, u8)> {
    use TokenKind::*;
    let bp = match op {
        Great | StarGreat | AmperGreat => (7, 8),
        Less | StarLess | AmperLess => (7, 8),

        Pipe | StarPipe | AmperPipe => (5, 6),

        AmperAmper => (3, 4),
        PipePipe => (1, 2),

        _ => return None,
    };
    Some(bp)
}
