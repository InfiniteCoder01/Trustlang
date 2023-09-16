use clap::Parser;
use std::io::{BufRead, Write};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// A trust lang compiler and interpreter
struct Cli {
    /// File to compile
    file: Option<String>,
}

pub fn compile<T: std::io::Read>(source: T, sourcepath: Option<&str>) {
    let (result, errors) = trustlang::parse(source, sourcepath);
    if errors.is_empty() {
        println!("Compiled successfully!\nResult: {result:?}");
    } else {
        eprintln!("Compiled with errors: errors{errors:?}!");
        println!("Result: {result:?}");
    }
}

fn main() {
    let cli = Cli::parse();
    if let Some(file) = cli.file {
        match std::fs::File::open(&file) {
            Ok(source) => compile(source, Some(&file)),
            Err(err) => eprintln!("Failed to open file {:?}: {}", file, err),
        }
    } else {
        loop {
            print!("> ");
            std::io::stdout().flush().unwrap();
            if let Some(line) = std::io::stdin()
                .lock()
                .lines()
                .next()
                .and_then(|line| line.ok())
            {
                compile(std::io::Cursor::new(line), None);
            } else {
                break;
            }
        }
    }
}
