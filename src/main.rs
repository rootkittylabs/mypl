use mypl::lexer::Lexer;
use mypl::parser::Parser;
fn main() {
    let input = "122 + 113   - 20 / 55 * 188 - \"zzzzz\" = 2".to_string();
    let mut lexer = Lexer::new(&input);
    let tokens = lexer.get_tokens().unwrap();

    let mut parser = Parser::new(&tokens);
    parser.parse();
}