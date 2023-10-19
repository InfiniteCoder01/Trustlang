#![warn(missing_docs)]
//! Trustlang - Rust with effect system and more accessible macros
//! Trustlang compiler also wants to achieve those goals:
//! - Be fast
//! - Be compatible with a lot of architectures
//! - Be able to call C, C++, Rust and maybe some others code
//! - Be able to generate functions, compatible with C, C++, Rust and maybe some others
//!

pub use logos::Logos;

/// Lexer, splits source into tokens - meaningful parts. See [lexer::Token] for more info
pub mod lexer;
/// Parser. Takes what lexer outputs, reads it and builds an IR - list of types, traits, functions, etc.
pub mod parser;

/// Codebase - holds the source of every file
pub type Codebase = codespan_reporting::files::SimpleFiles<String, std::rc::Rc<str>>;

/// TODO: A test function
pub fn parse(codebase: &mut Codebase) -> Option<()> {
    // let mut crate_ = parser::Crate::new();

    let mut file_id = 0;
    while let Ok(file) = codebase.get(file_id) {
        let name = file.name().clone();
        let source = file.source().clone();
        let mut lexer = lexer::Token::lexer(&source);
        dbg!(lexer.collect::<Vec<_>>());
        // parser::item::module::expect_entire(&mut tokens, &mut crate_, &parser::Path::new(&[name]));
        file_id += 1;
    }
    None
    // if codebase.errors() > 0 {
    //     None
    // } else {
    //     Some(crate_)
    // }
}

// #[macro_export]
// macro_rules! bug_result {
//     ($codebase: expr, $expr: expr, $message: expr$(, $labels: expr)?) => {
//         match $expr
//         {
//             Some(value) => value,
//             None => {
//                 $codebase.emit(
//                     Diagnostic::bug()
//                         .with_message($message)
//                         $(.with_labels($labels))?,
//                 );
//                 Default::default()
//             }
//         }
//     };
// }
