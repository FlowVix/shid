use crate::sources::span::Spanned;

use super::operators::{BinOp, UnaryOp};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    IntegerLiteral(u128),
    FloatLiteral(f64),
    StringLiteral(String),

    Ident(String),

    BinOp(Box<Spanned<Expr>>, BinOp, Box<Spanned<Expr>>),
    UnaryOp(UnaryOp, Box<Spanned<Expr>>),

    Block(Box<Spanned<Block>>),

    Array(Vec<Spanned<Expr>>),
    Tuple(Vec<Spanned<Expr>>),

    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub normal: Vec<Spanned<Stmt>>,
    pub ret: Option<Spanned<Stmt>>,
}

// pub enum Pattern {
//     Variable,
// }
