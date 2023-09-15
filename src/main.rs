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
            Ok(source) => match trustlang::parse(source, Some(&file)) {
                Ok(result) => println!("{:?}", result),
                Err(error) => eprintln!("{}", error),
            },
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
                match trustlang::parse(std::io::Cursor::new(line), None) {
                    Ok(result) => println!("{:?}", result),
                    Err(error) => eprintln!("{}", error),
                }
            } else {
                break;
            }
        }
    }
}
