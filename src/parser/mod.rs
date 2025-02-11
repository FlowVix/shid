use ast::{Block, Expr, Stmt};
use error::ParserError;
use lexer::{Lexer, Token};
use warning::ParserWarning;

use crate::sources::{
    span::{CodeArea, Span, Spannable, Spanned},
    Source, SourceKey, SourceMap,
};

pub mod ast;
pub mod error;
pub mod lexer;
pub mod operators;
pub mod warning;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    // prev: Option<(Token, Span)>,
    pub src: SourceKey,
    pub errors: Vec<ParserError>,
    pub warnings: Vec<ParserWarning>,
}

impl<'a> Parser<'a> {
    pub fn new(src: Source, sources: &'a mut SourceMap) -> Self {
        let key = sources.insert(src);
        Self {
            lexer: Lexer::new(&sources[key].content),
            // prev: None,
            src: key,
            errors: vec![],
            warnings: vec![],
        }
    }

    fn error(&mut self, error: ParserError) {
        self.errors.push(error);
    }
    fn warn(&mut self, warning: ParserWarning) {
        self.warnings.push(warning);
    }

    fn next_tok(&mut self) -> Token {
        self.lexer.next()
    }
    fn peek_tok(&self) -> Token {
        self.lexer.clone().next()
    }
    fn peek_toks<const N: usize>(&self) -> [Token; N] {
        let mut l = self.lexer.clone();
        std::array::from_fn(|_| l.next())
    }
    fn next_is(&self, tok: Token) -> bool {
        self.peek_tok() == tok
    }
    fn skip_tok(&mut self, tok: Token) -> bool {
        if self.next_is(tok) {
            self.next_tok();
            true
        } else {
            false
        }
    }

    fn expect_tok_named(&mut self, tok: Token, name: &str) {
        let peek = self.peek_tok();
        if peek == tok {
            self.next_tok();
        } else {
            self.error(ParserError::Expected {
                expected: name.to_string(),
                found: peek,
                area: self.area(self.peek_span()),
            });
        }
    }
    fn expect_tok(&mut self, tok: Token) {
        self.expect_tok_named(tok, &format!("`{}`", tok.name()));
    }

    fn area(&self, span: Span) -> CodeArea {
        span.to_area(self.src)
    }

    fn span(&self) -> Span {
        self.lexer.span()
    }
    fn slice(&self) -> &str {
        self.lexer.slice()
    }

    fn peek_span(&self) -> Span {
        let mut l = self.lexer.clone();
        l.next();
        l.span()
    }

    /// meant to be called after passing the opening token
    pub fn list_parse<F: FnMut(&mut Self)>(&mut self, delim: Token, end: Token, mut cb: F) {
        loop {
            if self.skip_tok(end) {
                break;
            }
            cb(self);
            if !self.skip_tok(delim) {
                self.expect_tok(end);
                break;
            }
        }
    }

    pub fn parse_unit(&mut self) -> Spanned<Expr> {
        let unary;

        match self.peek_tok() {
            Token::Integer => {
                self.next_tok();
                Expr::IntegerLiteral(self.slice().parse().unwrap()).spanned(self.span())
            }
            Token::Float => {
                self.next_tok();
                Expr::FloatLiteral(self.slice().parse().unwrap()).spanned(self.span())
            }
            Token::Ident => {
                self.next_tok();
                Expr::Ident(self.slice().into()).spanned(self.span())
            }
            Token::OpenParen => {
                self.next_tok();
                let start = self.span();
                let inner = self.parse_expr();

                if self.skip_tok(Token::Comma) {
                    let mut v = vec![inner];

                    self.list_parse(Token::Comma, Token::CloseParen, |slef| {
                        v.push(slef.parse_expr());
                    });

                    Expr::Tuple(v).spanned(start.extended(self.span()))
                } else {
                    self.expect_tok(Token::CloseParen);
                    inner.val.spanned(start.extended(self.span()))
                }
            }
            Token::OpenSquare => {
                self.next_tok();
                let start = self.span();

                let mut v = vec![];

                self.list_parse(Token::Comma, Token::CloseSquare, |slef| {
                    v.push(slef.parse_expr());
                });

                Expr::Array(v).spanned(start.extended(self.span()))
            }
            Token::OpenCurly => {
                self.next_tok();
                let start = self.span();
                let block = self.parse_block();
                Expr::Block(Box::new(block)).spanned(start.extended(self.span()))
            }
            unary_op
                if {
                    unary = operators::unary_prec(unary_op);
                    unary.is_some()
                } =>
            {
                self.next_tok();
                let start = self.span();
                let unary_prec = unary.unwrap();
                let next_prec = operators::next_infix(unary_prec);
                let val = match next_prec {
                    Some(next_prec) => self.parse_op(next_prec),
                    None => self.parse_value(),
                };

                Expr::UnaryOp(unary_op.to_unary_op().unwrap(), Box::new(val))
                    .spanned(start.extended(self.span()))
            }
            t => {
                self.error(ParserError::Expected {
                    expected: "expression".into(),
                    found: t,
                    area: self.area(self.peek_span()),
                });
                Expr::Error.spanned(self.peek_span())
            }
        }
    }
    pub fn parse_value(&mut self) -> Spanned<Expr> {
        self.parse_unit()
    }
    pub fn parse_op(&mut self, prec: usize) -> Spanned<Expr> {
        let next_prec = operators::next_infix(prec);

        let mut left = match next_prec {
            Some(next_prec) => self.parse_op(next_prec),
            None => self.parse_value(),
        };

        while operators::is_infix_prec(self.peek_tok(), prec) {
            let op = self.next_tok();

            let right = if operators::prec_type(prec) == operators::OpType::Left {
                match next_prec {
                    Some(next_prec) => self.parse_op(next_prec),
                    None => self.parse_value(),
                }
            } else {
                self.parse_op(prec)
            };
            let new_span = left.span.extended(right.span);
            left = Expr::BinOp(Box::new(left), op.to_bin_op().unwrap(), Box::new(right))
                .spanned(new_span)
        }

        left
    }
    pub fn parse_expr(&mut self) -> Spanned<Expr> {
        self.parse_op(0)
    }
    /// meant to be called after passing the opening brace
    pub fn parse_block(&mut self) -> Spanned<Block> {
        let start = self.span();
        let mut block = Block {
            normal: vec![],
            ret: None,
        };

        loop {
            let expr = self.parse_expr();
            let stmt = expr.map(Stmt::Expr);
            if !self.skip_tok(Token::Semicolon) {
                self.expect_tok(Token::CloseCurly);
                block.ret = Some(stmt);
                return block.spanned(start.extended(self.span()));
            }
            block.normal.push(stmt);
            if self.skip_tok(Token::CloseCurly) {
                return block.spanned(start.extended(self.span()));
            }
        }
    }
    pub fn parse_cock(&mut self) -> Spanned<Block> {
        self.expect_tok(Token::OpenCurly);
        let out = self.parse_block();
        self.expect_tok(Token::Eof);
        out
    }
}
