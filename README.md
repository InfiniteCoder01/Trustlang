# Trustlang - Rust with effect system and more accessible macros
Trustlang compiler also wants to achieve those goals:
- Be fast
- Be compatible with a lot of architectures
- Be able to call C, C++, Rust and maybe some others code
- Be able to generate functions, compatible with C, C++, Rust and maybe some others

# Compilation process
Compilation process is intended to be fast and simple, works in 2 passes:
- Pass 1:
    - Lazy tokenization
    - Macro invocation
    - Parsing AST
    - Name indexing
    - Type inference
- Pass 2 (traverses AST, runs on macros immediately):
    - Type checking
    - Operator overloading
    - Borrow checking
    - Backend invokation (compilation & optimization)
