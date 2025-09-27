//! Abstract syntax tree types

use std::ops::Range;

#[derive(Debug, PartialEq, Clone)]
pub struct Expr {
    kind: Box<ExprKind>,
    span: Range<usize>,
}

impl Expr {
    pub fn new(kind: ExprKind, span: Range<usize>) -> Self {
        Self {
            kind: Box::new(kind),
            span,
        }
    }

    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExprKind {
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
    pub lt: Box<Expr>,
    pub op: OpInfix,
    pub rt: Box<Expr>,
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
