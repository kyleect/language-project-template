//! Abstract syntax tree types

use std::ops::Range;

#[derive(Debug, PartialEq, Clone)]
pub struct Expr {
    kind: Box<ExprKind>,
    span: Range<usize>,
}

impl Expr {
    pub fn new<T: Into<ExprKind>>(kind: T, span: Range<usize>) -> Self {
        Self {
            kind: Box::new(kind.into()),
            span,
        }
    }

    pub fn error() -> Self {
        Self {
            kind: Box::new(ExprKind::Error),
            span: 0..0,
        }
    }

    /// Get the expression's span
    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }

    /// Get the expression's span start
    pub fn start(&self) -> usize {
        self.span().start
    }

    /// Get the expression's span end
    pub fn end(&self) -> usize {
        self.span().end
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExprKind {
    Literal(Box<ExprLiteral>),
    InfixOp(Box<ExprInfixOp>),
    Error,
}

impl ExprKind {
    pub fn infix_op(lt: Expr, op: OpInfix, rt: Expr) -> Self {
        Self::InfixOp(Box::new(ExprInfixOp {
            lt: lt.into(),
            op,
            rt: rt.into(),
        }))
    }
}

impl From<f64> for ExprKind {
    fn from(value: f64) -> Self {
        Self::Literal(Box::new(ExprLiteral::Number(value)))
    }
}

impl From<(Expr, OpInfix, Expr)> for ExprKind {
    fn from(value: (Expr, OpInfix, Expr)) -> Self {
        Self::infix_op(value.0, value.1, value.2)
    }
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
