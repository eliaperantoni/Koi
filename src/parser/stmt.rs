use crate::ast::{Expr, Stmt};
use crate::token::{Token, TokenKind};
use crate::interp::Func;

use super::Parser;

impl Parser {
    pub fn parse_stmt(&mut self) -> Stmt {
        match self.lexer.peek() {
            Some(Token {kind: TokenKind::Let, ..}) |
            Some(Token {kind: TokenKind::Exp, ..}) => self.parse_let_stmt(),

            Some(Token{kind: TokenKind::If, ..}) => self.parse_if_stmt(),
            Some(Token{kind: TokenKind::For, ..}) => self.parse_for_stmt(),
            Some(Token{kind: TokenKind::While, ..}) => self.parse_while_stmt(),
            Some(Token{kind: TokenKind::Fn, ..}) => self.parse_fn_stmt(),

            Some(Token{kind: TokenKind::Return, ..}) => self.parse_return(),

            Some(Token{kind: TokenKind::Continue, ..}) => {
                self.lexer.next();
                Stmt::Continue
            },
            Some(Token{kind: TokenKind::Break, ..}) => {
                self.lexer.next();
                Stmt::Break
            },

            _ => {
                let is_dollar_in_front = matches!(self.lexer.peek(), Some(Token {kind: TokenKind::Dollar, ..}));
                if !self.is_expr_next() || is_dollar_in_front {
                    if is_dollar_in_front {
                        // Consume the dollar
                        self.lexer.next();
                    }

                    let was_multiline = self.is_multiline;
                    self.is_multiline = false;
                    let cmd = Stmt::Cmd(self.parse_cmd(0));

                    if !self.is_at_end() {
                        panic!("expected newline or EOF");
                    }

                    self.is_multiline = was_multiline;

                    cmd
                } else {
                    let expr = self.parse_expr(0);
                    if !matches!(expr, Expr::Set(..) | Expr::SetField {..} | Expr::Call {..} | Expr::Cmd(..)) {
                        panic!("only assignment, call and command expressions are allowed as statements");
                    }
                    Stmt::Expr(expr)
                }
            }
        }
    }

    fn is_expr_next(&mut self) -> bool {
        if !self.lexer.is_new_line {
            return true;
        }

        if matches!(self.lexer.peek(), Some(Token{kind: TokenKind::DollarLeftParen, ..})) {
            return true;
        }

        self.lexer.start_recording();

        let line_tokens = self.lexer
            .by_ref()
            .take_while(|t| t.kind != TokenKind::Newline)
            .filter(|t| t.kind != TokenKind::Space)
            .collect::<Vec<Token>>();

        self.lexer.stop_recording(true);

        let mut line_tokens_iter = line_tokens.iter().peekable();

        loop {
            if !matches!(line_tokens_iter.next(), Some(Token {kind: TokenKind::Identifier(..), ..})) {
                return false;
            }

            if matches!(line_tokens_iter.peek(), Some(&Token {kind: TokenKind::Dot, ..})) {
                line_tokens_iter.next();
                continue;
            }

            return matches!(line_tokens_iter.next(),
                Some(Token {kind: TokenKind::LeftParen, ..}) |
                Some(Token {kind: TokenKind::LeftBracket, ..}) |
                Some(Token {kind: TokenKind::Equal, ..})
            );
        }
    }

    fn parse_let_stmt(&mut self) -> Stmt {
        let is_exp = self.lexer.peek().unwrap().kind == TokenKind::Exp;

        if is_exp {
            self.lexer.next();
            self.lexer.consume_whitespace(self.is_multiline);
        }

        // Only meaningful if there was an `exp`. Otherwise this has already been checked by `parse_stmt`
        if !matches!(self.lexer.next(), Some(Token{kind: TokenKind::Let, ..})) {
            panic!("expected let");
        }

        self.lexer.consume_whitespace(self.is_multiline);

        let name = self.must_identifier();

        self.lexer.consume_whitespace(self.is_multiline);

        if matches!(self.lexer.peek(), Some(Token{kind: TokenKind::Equal, ..})) {
            self.lexer.next();
            self.lexer.consume_whitespace(self.is_multiline);
            let init = Some(self.parse_expr(0));

            Stmt::Let {
                is_exp,
                name,
                init,
            }
        } else {
            Stmt::Let {
                is_exp,
                name,
                init: None,
            }
        }
    }

    pub fn parse_block(&mut self) -> Stmt {
        self.lexer.next();
        let stmts = self.parse_stmts();

        if !matches!(self.lexer.next(), Some(Token {kind: TokenKind::RightBrace, ..})) {
            panic!("expected right brace");
        }

        Stmt::Block(stmts)
    }

    fn parse_if_stmt(&mut self) -> Stmt {
        self.lexer.next();

        self.lexer.consume_whitespace(self.is_multiline);
        let cond = self.parse_expr(0);

        self.lexer.consume_whitespace(self.is_multiline);
        let then_do = Box::new(self.parse_block());

        self.lexer.consume_whitespace(self.is_multiline);
        let else_do = if matches!(self.lexer.peek(), Some(Token{kind: TokenKind::Else, ..})) {
            self.lexer.next();
            self.lexer.consume_whitespace(self.is_multiline);

            let else_do = if matches!(self.lexer.peek(), Some(Token{kind: TokenKind::If, ..})) {
                self.parse_if_stmt()
            } else {
                self.parse_block()
            };

            Some(Box::new(else_do))
        } else {
            None
        };

        Stmt::If {
            cond,
            then_do,
            else_do,
        }
    }

    fn parse_for_stmt(&mut self) -> Stmt {
        self.lexer.next();

        self.lexer.consume_whitespace(self.is_multiline);
        let key_var = self.must_identifier();

        self.lexer.consume_whitespace(self.is_multiline);
        if !matches!(self.lexer.next(), Some(Token{kind: TokenKind::Comma, ..})) {
            panic!("expected comma");
        }

        self.lexer.consume_whitespace(self.is_multiline);
        let val_var = self.must_identifier();

        self.lexer.consume_whitespace(self.is_multiline);
        if !matches!(self.lexer.next(), Some(Token{kind: TokenKind::In, ..})) {
            panic!("expected in");
        }

        self.lexer.consume_whitespace(self.is_multiline);
        let iterated = self.parse_expr(0);

        self.lexer.consume_whitespace(self.is_multiline);
        let each_do = self.parse_block();

        Stmt::For {
            iterated,
            each_do: Box::new(each_do),
            key_var,
            val_var,
        }
    }

    fn parse_while_stmt(&mut self) -> Stmt {
        self.lexer.next();

        self.lexer.consume_whitespace(self.is_multiline);
        let cond = self.parse_expr(0);

        self.lexer.consume_whitespace(self.is_multiline);
        let then_do = self.parse_block();

        Stmt::While {
            cond,
            then_do: Box::new(then_do),
        }
    }

    fn parse_fn_stmt(&mut self) -> Stmt {
        self.lexer.next();

        self.lexer.consume_whitespace(self.is_multiline);
        let name = self.must_identifier();

        self.lexer.consume_whitespace(self.is_multiline);

        let (params, body) = match self.continue_parse_fn() {
            Func::User {params, body, ..} => (params, body),
            _ => unreachable!(),
        };

        let func = Func::User {
            name: Some(name),
            params,
            body,
        };

        Stmt::Func(func)
    }

    fn parse_return(&mut self) -> Stmt {
        self.lexer.next();

        self.lexer.consume_whitespace(false);
        if matches!(self.lexer.peek(), None | Some(Token {kind: TokenKind::Newline, ..})) {
            return Stmt::Return(None);
        }

        let expr = self.parse_expr(0);
        Stmt::Return(Some(expr))
    }
}
