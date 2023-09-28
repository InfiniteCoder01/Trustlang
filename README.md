# Trustlang - Rust with effect system and more accessible macros
Trustlang compiler also wants to achieve those goals:
- Be fast
- Be compatible with a lot of architectures
- Be able to call C, C++, Rust and maybe some others code
- Be able to generate functions, compatible with C, C++, Rust and maybe some others

# Compilation process
Compilation process is intended to be fast and simple:
- Pass 1:
    - \[Kinda frontended] Lazy tokenization
    - Macro invocation
    - Parsing AST
    - Name indexing
    - Type inference
- Pass 2 (traverses AST, runs on macros lazely):
    - Type checking
    - Operator overloading
    - Borrow checking
    - \[Backend] IR
- Backend work (compilation & optimization)
