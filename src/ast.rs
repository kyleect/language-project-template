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

    pub fn kind(&self) -> &ExprKind {
        &self.kind
    }
}

#[cfg(test)]
mod expr_tests {
    use super::*;

    #[test]
    fn test_expr_new_number() {
        let expr = Expr::new(1.0, 0..5);

        assert_eq!(
            ExprKind::Literal(ExprLiteral::Number(1.0).into()),
            *expr.kind()
        );
        assert_eq!(0..5, expr.span());
    }

    #[test]
    fn test_expr_new_infix_op() {
        let expr = Expr::new(
            (Expr::new(1.0, 0..1), OpInfix::Add, Expr::new(2.0, 2..3)),
            0..3,
        );

        assert_eq!(
            ExprKind::InfixOp(
                ExprInfixOp {
                    lt: Expr::new(1.0, 0..1).into(),
                    op: OpInfix::Add,
                    rt: Expr::new(2.0, 2..3).into()
                }
                .into()
            ),
            *expr.kind()
        );
        assert_eq!(0..3, expr.span());
    }

    #[test]
    fn test_expr_error() {
        let expr = Expr::error();

        assert_eq!(ExprKind::Error, *expr.kind());
        assert_eq!(0..0, expr.span());
    }

    #[test]
    fn test_expr_span() {
        let expr = Expr::new(ExprKind::Error, 2..7);
        assert_eq!(2..7, expr.span());
    }

    #[test]
    fn test_expr_start() {
        let expr = Expr::new(ExprKind::Error, 2..7);
        assert_eq!(2, expr.start());
    }

    #[test]
    fn test_expr_end() {
        let expr = Expr::new(ExprKind::Error, 2..7);
        assert_eq!(7, expr.end());
    }

    #[test]
    fn test_expr_kind() {
        let kind = ExprKind::Literal(Box::new(ExprLiteral::Number(1.0)));
        let expr = Expr::new(kind.clone(), 0..5);
        assert_eq!(kind, *expr.kind());
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExprKind {
    Literal(Box<ExprLiteral>),
    InfixOp(Box<ExprInfixOp>),
    Error,
}

impl From<f64> for ExprKind {
    fn from(value: f64) -> Self {
        Self::Literal(Box::new(ExprLiteral::Number(value)))
    }
}

impl From<(Expr, OpInfix, Expr)> for ExprKind {
    fn from(value: (Expr, OpInfix, Expr)) -> Self {
        Self::InfixOp(
            ExprInfixOp {
                lt: value.0.into(),
                op: value.1,
                rt: value.2.into(),
            }
            .into(),
        )
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
