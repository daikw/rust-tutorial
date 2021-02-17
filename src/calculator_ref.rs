mod components {
  #[derive(Debug, Clone, PartialEq, Eq, Hash)]
  struct Loc(usize, usize);

  impl Loc {
    fn merge(&self, other: &Loc) -> Loc {
      use std::cmp::{max, min};
      Loc(min(self.0, other.0), max(self.1, other.1))
    }
  }

  #[derive(Debug, Clone, PartialEq, Eq, Hash)]
  struct Annot<T> {
    value: T,
    loc: Loc,
  }

  impl<T> Annot<T> {
    fn new(value: T, loc: Loc) -> Annot<T> {
      Self { value, loc }
    }
  }

  #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
  enum TokenKind {
    Number(u64), //[1-9][0-9]*
    Plus,        // +
    Minus,       // -
    Asterisk,    // *
    Slash,       // /
    LParen,      // (
    RParen,      // )
  }

  type Token = Annot<TokenKind>;
  impl Token {
    fn number(n: u64, loc: Loc) -> Self {
      Self::new(TokenKind::Number(n), loc)
    }

    fn plus(loc: Loc) -> Self {
      Self::new(TokenKind::Plus, loc)
    }

    fn minus(loc: Loc) -> Self {
      Self::new(TokenKind::Minus, loc)
    }

    fn asterisk(loc: Loc) -> Self {
      Self::new(TokenKind::Asterisk, loc)
    }

    fn slash(loc: Loc) -> Self {
      Self::new(TokenKind::Slash, loc)
    }

    fn lparen(loc: Loc) -> Self {
      Self::new(TokenKind::LParen, loc)
    }

    fn rparen(loc: Loc) -> Self {
      Self::new(TokenKind::RParen, loc)
    }
  }

  #[derive(Debug, Clone, PartialEq, Eq, Hash)]
  enum LexErrorKind {
    InvalidChar(char),
    Eof,
  }

  type LexError = Annot<LexErrorKind>;
  impl LexError {
    fn invalid_char(c: char, loc: Loc) -> Self {
      Self::new(LexErrorKind::InvalidChar(c), loc)
    }

    fn eof(loc: Loc) -> Self {
      Self::new(LexErrorKind::Eof, loc)
    }
  }

  fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let input = input.as_bytes();

    let mut tokens = Vec::new();
    let mut pos = 0;

    macro_rules! lex_a_token {
      ($lexer:expr) => {{
        let (tok, p) = $lexer?;
        tokens.push(tok);
        pos = p;
      }};
    }

    while pos < input.len() {
      match input[pos] {
        b'0'..=b'9' => lex_a_token!(lex_number(input, pos)),
        b'+' => lex_a_token!(lex_plus(input, pos)),
        b'-' => lex_a_token!(lex_minus(input, pos)),
        b'*' => lex_a_token!(lex_asterisk(input, pos)),
        b'/' => lex_a_token!(lex_slash(input, pos)),
        b'(' => lex_a_token!(lex_lparen(input, pos)),
        b')' => lex_a_token!(lex_rparen(input, pos)),
        b' ' | b'\n' | b'\t' => {
          let ((), p) = skip_spaces(input, pos);
          pos = p;
        }
        b => lex_a_token!(lex_plus(input, pos)),
      }
    }

    Ok(tokens)
  }

  fn consume_byte(input: &[u8], pos: usize, b: u8) -> Result<(u8, usize), LexError> {
    if input.len() <= pos {
      return Err(LexError::eof(Loc(pos, pos)));
    }

    if input[pos] != b {
      return Err(LexError::invalid_char(
        input[pos] as char,
        Loc(pos, pos + 1),
      ));
    }

    Ok((b, pos + 1))
  }

  fn lex_plus(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'+').map(|(_, end)| (Token::plus(Loc(start, end)), end))
  }
  fn lex_minus(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'-').map(|(_, end)| (Token::minus(Loc(start, end)), end))
  }
  fn lex_asterisk(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'*').map(|(_, end)| (Token::asterisk(Loc(start, end)), end))
  }
  fn lex_slash(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'/').map(|(_, end)| (Token::slash(Loc(start, end)), end))
  }
  fn lex_lparen(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'(').map(|(_, end)| (Token::lparen(Loc(start, end)), end))
  }
  fn lex_rparen(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b')').map(|(_, end)| (Token::rparen(Loc(start, end)), end))
  }

  fn lex_number(input: &[u8], mut pos: usize) -> Result<(Token, usize), LexError> {
    use std::str::from_utf8;

    let start = pos;
    while pos < input.len() && b"1234556780".contains(&input[pos]) {
      pos += 1;
    }
    let n = from_utf8(&input[start..pos]).unwrap().parse().unwrap();

    Ok((Token::number(n, Loc(start, pos)), pos))
  }

  fn skip_spaces(input: &[u8], mut pos: usize) -> ((), usize) {
    while pos < input.len() && b" \n\t".contains(&input[pos]) {
      pos += 1;
    }
    ((), pos)
  }

  #[test]
  fn test_lexer() {
    assert_eq!(
      lex("1+1"),
      Ok(vec![
        Token::number(1, Loc(0, 1)),
        Token::plus(Loc(1, 2)),
        Token::number(1, Loc(2, 3))
      ])
    );

    assert_eq!(
      lex("1 + 2 * 3 - -10"),
      Ok(vec![
        Token::number(1, Loc(0, 1)),
        Token::plus(Loc(2, 3)),
        Token::number(2, Loc(4, 5)),
        Token::asterisk(Loc(6, 7)),
        Token::number(3, Loc(8, 9)),
        Token::minus(Loc(10, 11)),
        Token::minus(Loc(12, 13)),
        Token::number(10, Loc(13, 15)),
      ])
    )
  }
}
