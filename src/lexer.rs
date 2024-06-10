use crate::token::Token;
struct Lexer {
  input: Vec<char>,
}

impl Lexer {
  pub fn new(input: &str) -> Lexer {
    todo!();
  }
  fn next_token() -> Token {
    todo!();
  }
}
#[cfg(test)]

mod test {
  use crate::token::{Token, TokenKind};
  
  #[test]
  fn test_next_token() {

    let input: &str = "=+(){},;";

    let expected: Vec<Token> = vec![
      Token{
        kind: TokenKind::Assign,
        literal: "=".to_string(),
      },
      Token{
        kind: TokenKind::Plus,
        literal: "+".to_string(),
      },
      Token{
        kind: TokenKind::LParen,
        literal: "(".to_string(),
      },
      Token{
        kind: TokenKind::RParen,
        literal: ")".to_string(),
      },
      Token{
        kind: TokenKind::LBrace,
        literal: "{".to_string(),
      },
      Token{
        kind: TokenKind::RBrace,
        literal: "}".to_string(),
      },
      Token{
        kind: TokenKind::Comma,
        literal: ",".to_string(),
      },
      Token{
        kind: TokenKind::Semicolon,
        literal: ";".to_string(),
      },
      Token{
        kind: TokenKind::Eof,
        literal: "".to_string(),
      },
    ];

    let lexer: Lexer = Lexer::new(input);

    for (idx, exp_token) in expected.into_iter().enumerate() {
      let recv_token: Token = lexer.next_token();
      assert_eq!(exp_token.kind, recv_token.kind);
    }
  }
}