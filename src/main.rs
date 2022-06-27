use mypl::lexer::Lexer;
fn main() {
    let mut lexer = Lexer::new("122 + 113   - 20 / 55 * 188 - \"zzzzz\" = 2".to_string());
    println!("{:?}", lexer.get_tokens().unwrap());
}