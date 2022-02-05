use super::tokens::{IdentKind, LexToken, LiteralKind, Token, TokenKind, Value};
use logos::Logos;

pub fn get_token_stream(raw_code: &str) -> Vec<Token> {
  let mut token_stream: Vec<Token> = Vec::new();
  let mut lex = LexToken::lexer(raw_code);

  loop {
    let lex_token = lex.next();

    let (kind, value) = match lex_token {
      Some(LexToken::Number(val)) => (
        TokenKind::Literal(LiteralKind::Number),
        Some(Value::Number(val)),
      ),
      Some(LexToken::Boolean(val)) => (
        TokenKind::Literal(LiteralKind::Boolean),
        Some(Value::Boolean(val)),
      ),
      Some(LexToken::String(val)) => (
        TokenKind::Literal(LiteralKind::String),
        Some(Value::String(val)),
      ),
      // Some(LexToken::Ident(val)) => (
      //   TokenKind::Ident(IdentKind::Variable),
      //   Some(Value::String(val)),
      // ),
      Some(LexToken::Let) => (TokenKind::Ident(IdentKind::Let), None),
      Some(LexToken::If) => (TokenKind::Ident(IdentKind::If), None),
      Some(LexToken::OpenParen) => (TokenKind::OpenParen, None),
      Some(LexToken::CloseParen) => (TokenKind::CloseParen, None),
      Some(LexToken::OpenBrace) => (TokenKind::OpenBrace, None),
      Some(LexToken::CloseBrace) => (TokenKind::CloseBrace, None),
      Some(LexToken::Minus) => (TokenKind::Minus, None),
      Some(LexToken::Plus) => (TokenKind::Plus, None),
      Some(LexToken::Divide) => (TokenKind::Divide, None),
      Some(LexToken::Multiply) => (TokenKind::Multiply, None),
      Some(LexToken::Greater) => (TokenKind::Greater, None),
      Some(LexToken::Less) => (TokenKind::Less, None),

      Some(LexToken::Whitespace) => continue,
      Some(LexToken::Error) => continue,

      None => break,
    };

    token_stream.push(Token { kind, value });
  }
  token_stream
}
