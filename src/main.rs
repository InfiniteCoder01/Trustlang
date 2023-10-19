use clap::Parser;
use trustlang::Codebase;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// A trust lang compiler and interpreter
struct Cli {
    /// File to compile
    file: Option<String>,
}

pub fn compile(codebase: &mut Codebase) {
    match trustlang::parse(codebase) {
        // Some(result) => println!("Compiled successfully!\nResult:\n{result}"),
        // None => eprintln!("Compilation Failed!"),
        _ => (),
    }
}

fn main() {
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    let mut cli = Cli::parse();
    cli.file = Some(String::from("test.tr"));
    let mut codebase = Codebase::new();
    if let Some(file) = cli.file {
        match std::fs::read_to_string(&file) {
            Ok(source) => {
                codebase.add(file, std::rc::Rc::from(source));
                compile(&mut codebase);
            }
            Err(err) => eprintln!("Failed to open file {file:?}: {err}"),
        }
    } else {
        loop {
            let readline = rl.readline("> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str()).unwrap();
                    codebase.add(String::from("<buffer>"), std::rc::Rc::from(line));
                    compile(&mut codebase);
                }
                Err(rustyline::error::ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(rustyline::error::ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {err:?}");
                    break;
                }
            }
        }
    }

    // let mut backend = orecc_back::backends::x86_64::X86_64::default();
    // backend.instruction(
    //     None,
    //     Some(orecc_back::backends::x86_64::REX::new(
    //         true, false, false, false,
    //     )),
    //     vec![0xc7],
    //     Some(orecc_back::backends::x86_64::ModRM::new(3, 0, 0)),
    //     None,
    //     None,
    //     Some(orecc_back::backends::x86_64::Immediate::Imm64(60)),
    // );
    // backend.instruction(
    //     None,
    //     Some(orecc_back::backends::x86_64::REX::new(
    //         true, false, false, false,
    //     )),
    //     vec![0xc7],
    //     Some(orecc_back::backends::x86_64::ModRM::new(3, 0, 7)),
    //     None,
    //     None,
    //     Some(orecc_back::backends::x86_64::Immediate::Imm64(42)),
    // );
    // backend.instruction(None, None, vec![0x0f, 0x05], None, None, None, None);
    // let mut out_file = std::fs::File::create("test.o").unwrap();
    // orecc_back::packaging::elf::pack(&mut out_file, target_lexicon::Triple::host(), &backend.assembly)
    //     .unwrap();
}
