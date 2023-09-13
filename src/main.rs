use clap::Parser;
use std::io::{BufRead, Write};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// A trust lang compiler and interpreter
struct Cli {
    /// File to compile
    file: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    if let Some(file) = cli.file {
        match std::fs::File::open(&file) {
            Ok(source) => println!("{:?}", trustlang::parse(source, Some(&file))),
            Err(err) => panic!("Failed to open file {:?}: {}", file, err), // TODO: Better errors
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
                println!("{:?}", trustlang::parse(std::io::Cursor::new(line), None));
            } else {
                break;
            }
        }
    }
}
