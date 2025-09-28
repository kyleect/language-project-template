# Getting Started

## Overview

- [x] Setup
  - [x] Installing dependencies
- [x] Project Layout
- [x] Lexing
  - [x] Logos Introduction
  - [x] Tokens
  - [x] Span Information
  - [x] Error Handling
- [ ] AST
  - [ ] Expr & ExprKind
  - [ ] Span Information
- [x] Parsing
  - [x] LALRPOP Introduction
  - [x] Grammar File Overview
  - [x] Using Our Lexer
- [ ] Error Handling
  - [ ] Lexing Errors
  - [ ] Parsing Errors
  - [ ] ExpyrResult
  - [ ] Error Recovery
    - [ ] Collect All The Errors
    - [ ] AST Error Nodes
- [x] Testing
  - [x] Specification Tests
    - [x] Valid Syntax Tests
      - [x] Expected Tokens
      - [x] Expected AST
    - [x] Invalid Syntax Tests
      - [x] Expected Tokens
      - [x] Expected Error
      - [x] Expected Diagnostic

## Setup

### Installing Dependencies

- [Rust 1.88](https://rust-lang.org/)
- [Just](https://just.systems/) - Task Runner
- [Docker](https://www.docker.com/) _Optional_

## Project Layout

### [`ast.rs`](./src/ast.rs)

This is where the abstract syntax tree structs and enums are defined. These are used in the [`grammar file`](./src/grammar.lalrpop) and in the [parser](./src/parser.rs).

In this starter:

- `Expr`: Representing expressions with a `ExprKind` and `Range<usize>` span.
- `ExprKind`: The type/kind of expression: number, infix operation
- `ExprLiteral`: Literal expression value like a number
- `ExprInfixOp`: Infix operation expression like addition or subtraction

### [`errors.rs`](./src/errors.rs)

This is where the `ExprError` and `ExprResult<T>` are defined. This is also where diagnostics logic live.

### [`grammar.lalrpop`](./src/grammar.lalrpop)

This is where the language's grammar is defined. It uses LALRPOP, which is a Rust like DSL for defining grammars, to produce a parsing function that's used by the [parser](#parserrs). It's configured to read in the tokens produced by the [lexer](#lexerrs)

### [`lexer.rs`](./src/lexer.rs)

This is where the lexical tokens are defined. Logos is used for this so it will automatically generate a lexer.

### [`parser.rs`](./src/parser.rs)

This is where the `parse` function lives. It wraps the [grammar file](#grammarlalrpop) produced parsing function to return `ExprResult`.

### [`span.rs`](./src/span.rs)

This is where the `Span` and `Spanned<T>` utility types are defined.

## Lexing

Lexing breaks text in to tokens e.g. `1 + 2` might become something like `Number(1)`, `Add`, `Number(2)` with each of those being a token.

Lexing is handled by the Rust crate [Logos](https://github.com/maciejhirsz/logos). This project's tokens are defined in [./src/lexer.rs](./src/lexer.rs).

```rust
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(error = Spanned<LexicalError>)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("(")]
    LParan,

    #[token(")")]
    RParan,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("/")]
    Slash,

    #[token("*")]
    Astrisk,

    #[regex(r#"[0-9]+(\.[0-9]+)?"#, lex_number)]
    Number(f64),
}
```

### Lexing Function

The `lex` function takes in a string source and returns a `Vec` of token results `Result<(usize, Token, usize), (ExprError, Span)>`.

This function is a wrapper around the `Lexer` that Logos produces.

```rust
use language_project_template::prelude::*;

let source = "1 + 2";

type TokenWithSpan = (usize, Token, usize);
type ErrorWithSpan = (ExprError, Span);

let tokens: Vec<Result<TokenWithSpan, ErrorWithSpan>> = lex(&source);
```

A successfully lexed token might be `Ok(0, Token::Number(1.0), 1)` where `0` and `1` are the `start` and `end` of the token's span respectively.

A token that can't be parsed might be `Err((ExprError::LexError(LexicalError::InvalidToken), 0..5))` where `0..5` is the span of the bad token.

#### Type

The result type of the lexer is defined in [./src/lexer.rs](./src/lexer.rs):

```rust
impl Iterator for Lexer<'_> {
  type Item = Result<(usize, Token, usize), ExprErrorS>;
}
```

### Whitespace

This project ignores whitespace by default when lexing.

```rust
#[logos(skip r"[ \t\n\f]+")]
```

Removing this line in [./src/lexer.rs](./src/lexer.rs) and adding token/s for whitespace is also possible. It depends on the needs of your language/parsing project.

## Abstract Syntax Tree (AST)

The AST is a simplified representation of the original source code. It's made up of a tree of nodes. Each node having a "type" e.g. statement, expression, module item. What node types you choose and how simplified (or abstracted) it is will depend on the needs of your language project. Things like comments and whitespace are abstent from the AST.

The AST, at least in this project, is the first attempt to modal your language's grammar in to something usable in code.

### What's It Used For?

ASTs can be used for code generation (IR, bytecode), static analysis, language servers, etc...

## Parsing

Parsing maps lexical tokens in to an abstract syntax tree (AST). This is the first stage where the rules of your grammar are exercised and enforced.

### LALRPOP

The parser is generated by [LALRPOP](https://lalrpop.github.io/lalrpop/) from the [grammar file](./src/grammar.lalrpop).

### Parsing Function

The `parse` function returns either the AST or a vector of parsing errors. It's a wrapper around the parser that LALRPOP generates.

```rust
use language_project_template::prelude::*;

let source = "1 + 2";

type ErrorWithSpan = (ExprError, Span);

let ast: Result<Expr, Vec<ErrorWithSpan>> = parse(&source);
```

## Testing

Language projects tend to need a health mix of low (unit tests) to high (integration, e2e) scoped tests to be effective. As the grammar/syntax grows, the project expands, more syntax features needing to be mixed/matched together, more and more tests are needed to prevent breaking things.

### Unit Tests

Unit tests live with the implementation code, the same as most Rust projects.

### Specification Tests

Specification tests live in [./spec/](./spec/) and are ran by [./tests/spec_tests.rs](./tests/spec_tests.rs). They are split between [valid](./spec/valid/) and [invalid](./spec/invalid/) syntax tests.

#### Valid Syntax Tests

The [valid](./spec/valid/) syntax tests consist of an input file `NAME.expr` along with expected results `NAME.expr.tokens` for the lexer and `NAME.expr.ast` for the parser. These are great for both catching bugs and helping identify where the problem is.

##### [./spec/valid/add.expr](./spec/valid/add.expr)

```
1 + 2
```

##### [./spec/valid/add.expr.tokens](./spec/valid/add.expr.tokens)

```
[
    Ok(
        (
            0,
            Number(
                1.0,
            ),
            1,
        ),
    ),
    Ok(
        (
            2,
            Plus,
            3,
        ),
    ),
    Ok(
        (
            4,
            Number(
                2.0,
            ),
            5,
        ),
    ),
]
```

##### [./spec/valid/add.expr.ast](./spec/valid/add.expr.ast)

```
Ok(
    Expr {
        kind: InfixOp(
            ExprInfixOp {
                lt: Expr {
                    kind: Literal(
                        Number(
                            1.0,
                        ),
                    ),
                    span: 0..1,
                },
                op: Add,
                rt: Expr {
                    kind: Literal(
                        Number(
                            2.0,
                        ),
                    ),
                    span: 4..5,
                },
            },
        ),
        span: 0..5,
    },
)
```

#### Invalid Syntax Tests

The [invalid](./spec/invalid/) syntax tests consist of an input file `NAME.expr` along with expected results `NAME.expr.tokens` for the lexer and expected error cases `NAME.expr.error` & `NAME.expr.diagnostics`. These are great for testing integrations like the language server or the VS code extension.

##### [./spec/invalid/unclosed_parans.expr](./spec/invalid/unclosed_parans.expr)

```
(
```

##### [./spec/invalid/unclosed_parans.expr.tokens](./spec/invalid/unclosed_parans.expr.tokens)

```
[
    Ok(
        (
            0,
            LParan,
            1,
        ),
    ),
]
```

##### [./spec/invalid/unclosed_parans.expr.error](./spec/invalid/unclosed_parans.expr.error)

```
Err(
    [
        (
            SyntaxError(
                UnrecognizedEOF {
                    expected: [
                        "\"(\"",
                        "number",
                    ],
                },
            ),
            1..1,
        ),
    ],
)
```

##### [./spec/invalid/unclosed_parans.expr.diagnostics](./spec/invalid/unclosed_parans.expr.diagnostics)

```
[
    Diagnostic {
        severity: Error,
        code: Some(
            "syntax",
        ),
        message: "unexpected end of file; expected: [\"\\\"(\\\"\", \"number\"]",
        labels: [
            Label {
                style: Primary,
                file_id: 0,
                range: 1..1,
                message: "",
            },
        ],
        notes: [],
    },
]
```
