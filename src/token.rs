pub struct Token {
  pub kind: TokenKind,
  pub literal: String,
}

pub enum TokenKind {
  Illegal,
  Eof,
  // Identifiers + literals
  Ident,
  Int,
  // Operators
  Assign,
  Plus,

  // Delimiters
  Comma,
  Semicolon,
  LParen,
  RParen,
  LBrace,
  RBrace,
  // Keywords
  Function,
  Let,
}