use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// A trust lang compiler and interpreter
struct Cli {
    /// File to compile
    file: Option<String>,
}

pub fn compile<T: std::io::Read>(source: T, sourcepath: Option<&str>) {
    match trustlang::parse(source, sourcepath) {
        Ok(result) => println!("Compiled successfully!\nResult:\n{result}"),
        Err(errors) => {
            eprintln!("Compilation Failed!");
            for error in errors {
                eprintln!("{}", error)
            }
        }
    }
}

fn main() {
    // let cli = Cli::parse();
    // if let Some(file) = cli.file {
    //     match std::fs::File::open(&file) {
    //         Ok(source) => compile(source, Some(&file)),
    //         Err(err) => eprintln!("Failed to open file {:?}: {}", file, err),
    //     }
    // } else {
    //     loop {
    //         print!("> ");
    //         std::io::stdout().flush().unwrap();
    //         if let Some(line) = std::io::stdin()
    //             .lock()
    //             .lines()
    //             .next()
    //             .and_then(|line| line.ok())
    //         {
    //             compile(std::io::Cursor::new(line), None);
    //         } else {
    //             break;
    //         }
    //     }
    // }

    let mut backend = orecc_back::backends::x86_64::X86_64::default();
    backend.instruction(None, None, vec![0xc3], None, None, None, None);

    let mut out_file = std::fs::File::create("test.o").unwrap();
    orecc_back::packaging::elf::pack(&mut out_file, &backend.assembly).unwrap();
}
