use mypl::lexer::Lexer;
use mypl::parser::Parser;
fn main() {
    let mut lexer = Lexer::new("122 + 113   - 20 / 55 * 188 - \"zzzzz\" = 2".to_string());
    let tokens = lexer.get_tokens().unwrap();

    let mut parser = Parser::new(&tokens);
    parser.parse();
}