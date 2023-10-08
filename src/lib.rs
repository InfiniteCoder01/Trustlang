pub mod lexer;
pub mod parser;
pub use orecc_front::Codebase;

pub fn parse(codebase: &mut Codebase) -> Option<orecc_back::ir::Module> {
    let mut file_id = 0;
    while file_id < codebase.files().len() {
        let file = &codebase.files()[file_id];
        let name = file.name().clone();
        let mut tokens = lexer::TokenBuffer::new(codebase, file.source().clone(), file_id);
        let mut crate_ = parser::Crate::new();
        parser::item::module::expect_entire(&mut tokens, &mut crate_, &parser::Path::new(&[name]));
        dbg!(crate_);
        // let mut ir = orecc_back::ir::Module::default();
        // loop {
        //     if let Some(declaration) = parser::item::parse(&mut tokens, path) {
        //         dbg!(declaration);
        //         // declaration.build(&mut ir);
        //         // } else if let Some(got) = tokens.peek_token() {
        //         //     let message = format!("expected item, got {}!", got.token);
        //         //     tokens.error(message);
        //     } else {
        //         break;
        //     }
        // }
        file_id += 1;
    }
    None
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
