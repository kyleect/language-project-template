# language-project-template

My starter for language projects. Rust, LALRPop, and Logos. Includes lexer, parser, capturing spans, error handling & diagnostics, a language server and VS Code extension, example CLI with Dockerization.

## Project

- [Lexer](./src/lexer.rs)
- [Parser](./src/parser.rs), [Grammar](./src/grammer.lalrpop), [AST](./src/ast.rs)
- [Valid](./spec/valid/) & [Invalid](./spec/invalid) [Syntax Examples](./spec/)
- [Specification](./tests//spec_tests.rs) & Integration [Tests](#tests)

## Syntax

| Syntax                                  | Description      |
| --------------------------------------- | ---------------- |
| `1`, `100`                              | A number         |
| `5 + 5`, `25 - 10`, `10 * 10`, `20 / 5` | Infix operations |
| `(5 + 10) - 2`                          | Grouping         |

See [/spec](./spec/) for more syntax examples.

## Built With

- [lalrpop](https://github.com/lalrpop/lalrpop)
- [logos](https://github.com/maciejhirsz/logos)

## Tests

### Specification Tests

The [specification tests](./tests/spec_tests.rs) read in `*.expr` files in the [`spec/`](./spec) directory and compare the output against their corrosponding expected result files (e.g. `*.expr.tokens`). These [spec]((./spec) files are split between [valid](./spec/valid) and [invalid](./spec/invalid) examples.

### Coverage

Run `just coverage` or [coverage.sh](./coverage.sh) to generate a coverage report.
