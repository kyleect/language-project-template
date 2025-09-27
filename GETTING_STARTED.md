# Getting Started

## Overview

- [x] Setup
  - [x] Installing dependencies
- [x] Project Layout
- [ ] Modeling The Language
  - [ ] AST
  - [ ] Expr & ExprKind
  - [ ] Span Information
- [ ] Lexing
  - [ ] Logos Introduction
  - [ ] Tokens
  - [ ] Span Information
  - [ ] Error Handling
- [ ] Parsing
  - [ ] LALRPOP Introduction
  - [ ] Grammar File Overview
  - [ ] Using Our Lexer
- [ ] Error Handling
  - [ ] Lexing Errors
  - [ ] Parsing Errors
  - [ ] ExpyrResult
  - [ ] Error Recovery
    - [ ] Collect All The Errors
    - [ ] AST Error Nodes
- [ ] Testing
  - [x] Specification Tests
    - [x] Valid Syntax Tests
      - [x] Expected Tokens
      - [x] Expected AST
    - [x] Invalid Syntax Tests
      - [x] Expected Tokens
      - [x] Expected Error
      - [x] Expected Diagnostic
  - [ ] Coverage

## Setup

### Installing Dependencies

- [Rust 1.88](https://rust-lang.org/)
- [Just](https://just.systems/) - Task Runner
- [Docker](https://www.docker.com/) _Optional_

### Project Layout

#### [`ast.rs`](./src/ast.rs)

This is where the abstract syntax tree structs and enums are defined. These are used in the [`grammar file`](./src/grammar.lalrpop) and in the [parser](./src/parser.rs).

In this starter:

- `Expr`: Representing expressions with a `ExprKind` and `Range<usize>` span.
- `ExprKind`: The type/kind of expression: number, infix operation
- `ExprLiteral`: Literal expression value like a number
- `ExprInfixOp`: Infix operation expression like addition or subtraction

#### [`errors.rs`](./src/errors.rs)

This is where the `ExprError` and `ExprResult<T>` are defined. This is also where diagnostics logic live.

#### [`grammar.lalrpop`](./src/grammar.lalrpop)

This is where the language's grammar is defined. It uses LALRPOP, which is a Rust like DSL for defining grammars, to produce a parsing function that's used by the [parser](#parserrs). It's configured to read in the tokens produced by the [lexer](#lexerrs)

#### [`lexer.rs`](./src/lexer.rs)

This is where the lexical tokens are defined. Logos is used for this so it will automatically generate a lexer.

#### [`parser.rs`](./src/parser.rs)

This is where the `parse` function lives. It wraps the [grammar file](#grammarlalrpop) produced parsing function to return `ExprResult`.

#### [`span.rs`](./src/span.rs)

This is where the `Span` and `Spanned<T>` utility types are defined.

## Testing

Language projects tend to need a health mix of low (unit tests) to high (integration, e2e) scoped tests to be effective. As the grammar/syntax grows, the project expands, more syntax features needing to be mixed/matched together, more and more tests are needed to prevent breaking things.

### Unit Tests

Unit tests live with the implementation code, the same as most Rust projects.

### Specification Tests

Specification tests live in [./spec/](./spec/) and are ran by [./tests/spec_tests.rs](./tests/spec_tests.rs). They are split between [valid](./spec/valid/) and [invalid](./spec/invalid/) syntax tests.

#### Valid Syntax Tests

The [valid](./spec/valid/) syntax tests consist of an input file `NAME.expr` along with expected results `NAME.expr.tokens` for the lexer and `NAME.expr.ast` for the parser. These are great for both catching bugs and helping identify where the problem is.

#### Invalid Syntax Tests

The [invalid](./spec/invalid/) syntax tests consist of an input file `NAME.expr` along with expected results `NAME.expr.tokens` for the lexer and expected error cases `NAME.expr.error` & `NAME.expr.diagnostics`. These are great for testing integrations like the language server or the VS code extension.
