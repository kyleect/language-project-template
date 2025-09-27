use language_project_template::lexer::lex;
use language_project_template::{errors::ExprResult, parser::parse};
use rstest::rstest;
use std::{fs::read_to_string, path::PathBuf};

#[rstest]
fn spec_files_tokens(#[files("spec/**/*.expr")] path: PathBuf) -> ExprResult<()> {
    let expected_tokens_path = path.with_extension("expr.tokens");

    if expected_tokens_path.exists() {
        let expected_tokens =
            read_to_string(expected_tokens_path).expect("should be able to read file");

        let expr_source = read_to_string(&path).expect("should be able to read file");

        let tokens = lex(&expr_source);

        pretty_assertions::assert_eq!(format!("{tokens:#?}"), expected_tokens);
    }

    let expected_ast_path = path.with_extension("expr.ast");

    if expected_ast_path.exists() {
        let expected_ast = read_to_string(expected_ast_path).expect("should be able to read file");

        let expr_source = read_to_string(&path).expect("should be able to read file");

        let ast = parse(&expr_source);

        pretty_assertions::assert_eq!(format!("{ast:#?}"), expected_ast);
    }

    Ok(())
}
