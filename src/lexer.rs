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
    ];
  }
}