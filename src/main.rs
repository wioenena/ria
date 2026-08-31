use ria::checker::Checker;
use ria::lexing::Lexer;
use ria::parser::Parser;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if let Some(file) = args.get(1) {
        let content = std::fs::read_to_string(file).unwrap();
        let mut lexer = Lexer::new(&content);
        let tokens = match lexer.get_tokens() {
            Ok(tokens) => tokens,
            Err(e) => {
                eprintln!("lexer error: {e}");
                std::process::exit(1);
            }
        };

        let program = match Parser::new(tokens).parse() {
            Ok(program) => program,
            Err(e) => {
                eprintln!("parse error: {e}");
                std::process::exit(1);
            }
        };

        match Checker::new().check(&program) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("checker error: {e}");
                std::process::exit(1);
            }
        }

        dbg!(&program);
    } else {
        eprintln!("use ria <file.ria>")
    }
}
