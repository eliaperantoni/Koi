use crate::ast::{Cmd, CmdOp, Expr, Value};
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

use super::Parser;

impl Parser {
    pub fn parse_cmd(&mut self, min_bp: u8) -> Cmd {
        self.lexer.consume_whitespace(self.is_multiline);

        let mut lhs = if let Some(Token { kind: TokenKind::LeftParen, .. }) = self.lexer.peek() {
            self.lexer.next();
            let cmd = self.parse_cmd(0);

            if !matches!(self.lexer.next(), Some(Token { kind: TokenKind::RightParen, .. })) {
                panic!("expected right parenthesis");
            }

            cmd
        } else {
            self.parse_cmd_atom()
        };

        self.lexer.consume_whitespace(self.is_multiline);

        loop {
            let op = match self.lexer.peek() {
                Some(t @ Token { .. }) if t.is_cmd_op() => &t.kind,
                _ => break
            };

            let (l_bp, r_bp) = binding_power(op).unwrap();
            if l_bp < min_bp {
                break;
            }

            let op = self.lexer.next().unwrap().kind;
            let rhs = self.parse_cmd(r_bp);

            lhs = Cmd::Op(
                Box::new(lhs),
                match op {
                    TokenKind::PipePipe => CmdOp::Or,
                    TokenKind::AmperAmper => CmdOp::And,

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

    fn parse_cmd_atom(&mut self) -> Cmd {
        let mut segments = Vec::new();

        loop {
            let mut exprs = Vec::new();

            self.lexer.consume_whitespace(self.is_multiline);

            loop {
                if self.lexer.peek().is_none() {
                    break;
                }

                if {
                    let t = self.lexer.peek().unwrap();
                    t.is_cmd_op() || [TokenKind::Space, TokenKind::Newline, TokenKind::RightParen].contains(&t.kind)
                } {
                    break;
                }

                let expr = match self.lexer.next().unwrap() {
                    t @ Token { kind: TokenKind::String { .. }, .. } => self.continue_parse_string_expr(t),
                    Token { kind: TokenKind::LeftBrace, .. } => {
                        self.lexer.consume_whitespace(self.is_multiline);
                        let expr = self.parse_expression(0);
                        self.lexer.consume_whitespace(self.is_multiline);

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

            if exprs.len() > 0 {
                segments.push(exprs);
            } else {
                break;
            }
        }

        if segments.len() == 0 {
            panic!("empty command");
        }

        Cmd::Atom(segments)
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

impl Token {
    fn is_cmd_op(&self) -> bool {
        binding_power(&self.kind).is_some()
    }
}
