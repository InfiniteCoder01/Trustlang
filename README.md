![](./docs/README.svg)
# Trustlang - Rust with effect system and more accessible macros
- 

# Compilation process
Compilation process is intended to be fast and simple, works in 2 passes (you can do 1 pass but most of the languages will probably require 2 passes):
- Pass 1:
    - Lazy tokenization (orecc-front + your code)
    - Macro invocation (if you have macros)
    - Parsing AST (orecc-front + your code)
    - Name indexing
    - Type inference
- Pass 2 (traverses AST, run on macros immediately):
    - Type checking
    - Operator overloading
    - //
