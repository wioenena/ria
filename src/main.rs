use ria::analyze::Parser;
use ria::lexing::Lexer;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if let Some(file) = args.get(1) {
        let content = std::fs::read_to_string(file).unwrap();
        let mut lexer = Lexer::new(&content);
        let tokens = lexer.get_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let decls = parser.parse().unwrap();
        dbg!(decls);
    } else {
        eprintln!("use ria <file.ria>")
    }
}
