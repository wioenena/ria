use ria::codegen::{CodeGenerator, Typescript};
use ria::lexing::Lexer;
use ria::parser::Parser;
use ria::resolver::Resolver;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if let Some(file) = args.get(1) {
        let content = std::fs::read_to_string(file).unwrap();
        let mut lexer = Lexer::new(&content);
        let tokens = lexer.get_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let decls = parser.parse().unwrap();
        let resolver = Resolver::new(&decls);
        resolver.resolve().unwrap();
        let codegen = CodeGenerator::<Typescript>::new();
        println!("Input: {content}");
        println!();
        println!("Output: {}", codegen.generate(&decls));
    } else {
        eprintln!("use ria <file.ria>")
    }
}
