//! Parsing source code in to expressions

use lalrpop_util::lalrpop_mod;

use crate::{
    ast::{self},
    errors::{ExprResult, SyntaxError},
    lexer::lex,
    parser::grammar::ExprParser,
};

lalrpop_mod!(grammar);

/// Parse source code in to an [`ast::Expr`].
pub fn parse(source: &str) -> ExprResult<ast::Expr> {
    let tokens = lex(source);

    let mut errs = vec![];

    let expr_parser = ExprParser::new();

    let mut parser_errors = Vec::new();

    let expr = match expr_parser.parse(source, &mut parser_errors, tokens) {
        Ok(ast) => ast,
        Err(err) => {
            errs.push(SyntaxError::from_parser_error(err, source));
            ast::Expr::error()
        }
    };

    errs.extend(parser_errors);

    if errs.is_empty() { Ok(expr) } else { Err(errs) }
}

#[cfg(test)]
mod parse_tests {
    use crate::{errors::ExprError, parser::parse};

    #[test]
    fn invalid_parse_produces_error() {
        let result = parse("(").err().unwrap();

        pretty_assertions::assert_eq!(
            vec![(
                ExprError::SyntaxError(crate::errors::SyntaxError::UnrecognizedEOF {
                    expected: vec!["\"(\"".to_string(), "number".to_string()]
                }),
                1..1
            )],
            result
        );
    }
}
