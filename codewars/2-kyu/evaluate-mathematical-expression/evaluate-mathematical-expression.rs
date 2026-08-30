use Token::*;
​
// ======================================
// Token
// ======================================
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Num(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    UMinus,
}
​
// ======================================
// Tokenize
// ======================================
fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();