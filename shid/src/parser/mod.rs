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

    // /// meant to be called after passing the opening token
    // pub fn list_parse<F: FnMut(&mut Self)>(&mut self, delim: Token, end: Token, mut cb: F) {
    //     loop {
    //         if self.skip_tok(end) {
    //             break;
    //         }
    //         cb(self);
    //         if !self.skip_tok(delim) {
    //             self.expect_tok(end);
    //             break;
    //         }
    //     }
    // }
}

pub trait Parse {
    fn parse(parser: &mut Parser) -> Self;
}
