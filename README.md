# Trustlang - Rust with effect system and more accessible macros
Trustlang compiler also wants to achieve those goals:
- Be fast
- Be compatible with a lot of architectures
- Be able to call C, C++, Rust and maybe some others code
- Be able to generate functions, compatible with C, C++, Rust and maybe some others

# Stuff for developers:

### Compilation Process (Version 3, October 19, 2023):
- Lexer, Tokens -> Language IR
- Type inference on Language IR
- borrow checking, Type checking & backend (backend is single for all crates) invocation (maybe I should move backend invokation out because of generic overduplication?)

Inside backend:
- [Probably, Hopefully] Backend is just a trait, so this should invoke the actual platfrom backend to [mostly] generate Backend IR (and variable metadate here too!)
- If backend uses IR, convert IR to machine code / [more likely] Invoke packager
- Packanger's job - package symbols (given - symbol name, some metadata and binary for the symbol), for example ELFs.

### TODO and notes for development
Warnings:
[rustc](https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html)
[clippy](https://rust-lang.github.io/rust-clippy/master/index.html)

LSP:
[lsp-server](https://crates.io/crates/lsp-server)
[lsp-types](https://crates.io/crates/lsp-types)
