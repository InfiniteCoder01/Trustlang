pub mod lexer;
pub mod parser;
pub use orecc_front::Codebase;

pub fn parse(codebase: &mut Codebase) -> Option<parser::Crate> {
    let mut crate_ = parser::Crate::new();

    let mut file_id = 0;
    while file_id < codebase.files().len() {
        let file = &codebase.files()[file_id];
        let name = file.name().clone();
        let mut tokens = lexer::TokenBuffer::new(codebase, file.source().clone(), file_id);
        parser::item::module::expect_entire(&mut tokens, &mut crate_, &parser::Path::new(&[name]));
        file_id += 1;
    }
    if codebase.errors() > 0 {
        None
    } else {
        Some(crate_)
    }
}

#[macro_export]
macro_rules! bug_result {
    ($codebase: expr, $expr: expr, $message: expr$(, $labels: expr)?) => {
        match $expr
        {
            Some(value) => value,
            None => {
                $codebase.emit(
                    Diagnostic::bug()
                        .with_message($message)
                        $(.with_labels($labels))?,
                );
                Default::default()
            }
        }
    };
}
