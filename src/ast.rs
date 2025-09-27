//! Abstract syntax tree types

use crate::span::Spanned;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Literal(Box<ExprLiteral>),
    InfixOp(Box<ExprInfixOp>),
    Error,
}

/// Expression representing literal values e.g. `number`
#[derive(Clone, Debug, PartialEq)]
pub enum ExprLiteral {
    Number(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprInfixOp {
    pub lt: ExprS,
    pub op: OpInfix,
    pub rt: ExprS,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum OpInfix {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    LogicAnd,
    LogicOr,
}

pub type ExprS = Spanned<Expr>;
