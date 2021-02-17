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

  #[allow(dead_code)]
  fn lexer(formula: String) -> Vec<String> {
    formula.chars().map(|x| x.to_string()).collect()
  }

  #[cfg(test)]
  mod test {
    use super::*;

    #[test]
    fn test_lexer() {
      assert_eq!(lexer("1+1".to_string()), ["1", "+", "1"]);
    }
  }
}
