# ria

**ria** is an experimental schema definition language and (eventually) a
multi-target code generator. You write your data types once, in a small,
readable syntax, and ria will generate matching type definitions for
languages like Go, Rust, and TypeScript.

```ria
type User {
    name string,
    age u8
}

enum HumanLifeState {
    Alive,
    Dead
}
```

> ⚠️ **Status: early development.** The lexer, parser, and AST are working.
> Type-checking/resolution and code generation for target languages are not
> implemented yet. See [Roadmap](#roadmap) below.

## Why

Keeping the same data shape in sync across a Go backend, a Rust service, and
a TypeScript frontend usually means hand-writing (and hand-maintaining) the
same struct/interface three times. ria aims to let you define that shape
once and generate idiomatic, native type definitions for each target
language from a single source of truth.

## Current syntax

Two kinds of declarations are supported today:

- `type` — a struct-like record with named, typed fields.
- `enum` — a simple set of named values.

```ria
type User {
    name string,
    age u8
}

enum HumanLifeState {
    Alive,
    Dead
}
```

Comments start with `#` and run to the end of the line.

Supported field types today: `i8`, `u8`, `string`. Referencing another
declared `type`/`enum` as a field's type, optional types, and collection
types (arrays, maps, etc.) are planned but not implemented yet.

## Project structure

The project is organized as a small compiler pipeline:

```
src/
├── lexing/   # source text -> tokens
├── parser/   # tokens -> AST
├── ast/      # AST node definitions (Decl, TypeDecl, EnumDecl, ...)
└── types/    # the primitive type system
```

Each stage is intentionally isolated in its own module so new stages
(semantic analysis, code generation) can be added without disturbing the
existing ones.

## Building and running

This is a standard Cargo project.

```sh
cargo build
cargo test
cargo run -- examples/basic.ria
```

Running the CLI on a `.ria` file currently lexes and parses it, then prints
the resulting AST for inspection — there is no generated output yet.

## Roadmap

- [x] Lexer
- [x] Parser and AST
- [x] Type resolution (referencing other declared types/enums as field types)
- [ ] Custom output with attributes
- [ ] Semantic analysis / validation
- [ ] Code generation
  - [ ] TypeScript
  - [ ] Go
  - [ ] Rust
- [ ] CLI ergonomics (output paths, target selection, etc.)

## License

MIT — see [LICENSE.txt](LICENSE.txt) for the full text.
