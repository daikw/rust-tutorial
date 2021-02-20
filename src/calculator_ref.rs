// ref: https://github.com/ghmagazine/rustbook/tree/master/ch09/parser

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Loc(usize, usize);
impl Loc {
  fn merge(&self, other: &Loc) -> Loc {
    use std::cmp::{max, min};
    Loc(min(self.0, other.0), max(self.1, other.1))
  }
}

use std::fmt;
impl fmt::Display for Loc {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}-{}", self.0, self.1)
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Annot<T> {
  value: T,
  loc: Loc,
}
impl<T> Annot<T> {
  fn new(value: T, loc: Loc) -> Annot<T> {
    Self { value, loc }
  }
}

use ast::Ast;
use ast::ParseError;
use lexer::LexError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Error {
  Lexer(LexError),
  Parser(ParseError),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "parser error")
  }
}

impl From<LexError> for Error {
  fn from(e: LexError) -> Self {
    Error::Lexer(e)
  }
}

impl From<ParseError> for Error {
  fn from(e: ParseError) -> Self {
    Error::Parser(e)
  }
}

pub use std::error::Error as StdError;
impl StdError for LexError {}
impl StdError for ParseError {}
impl StdError for Error {
  fn source(&self) -> Option<&(dyn StdError + 'static)> {
    use self::Error::*;
    match self {
      Lexer(lex) => Some(lex),
      Parser(parse) => Some(parse),
    }
  }
}

use interpreter::InterpreterErrorKind;
impl StdError for InterpreterError {
  fn description(&self) -> &str {
    use self::InterpreterErrorKind::*;
    match self.value {
      DivisionByZero => "the right hand expression of the division evaluates to zero",
    }
  }
}

fn print_annot(input: &str, loc: Loc) {
  eprintln!("{}", input);
  eprintln!("{}{}", " ".repeat(loc.0), "^".repeat(loc.1 - loc.0))
}

impl Error {
  pub fn show_diagnostic(&self, input: &str) {
    use self::Error::*;
    use self::ParseError as P;
    use lexer::Token;

    let (e, loc): (&dyn StdError, Loc) = match self {
      Lexer(e) => (e, e.loc.clone()),
      Parser(e) => {
        let loc = match e {
          P::UnexpectedToken(Token { loc, .. })
          | P::NotExpression(Token { loc, .. })
          | P::NotOperator(Token { loc, .. })
          | P::UnclosedOpenParen(Token { loc, .. }) => loc.clone(),
          P::RedundantExpression(Token { loc, .. }) => Loc(loc.0, input.len()),
          P::Eof => Loc(input.len(), input.len() + 1),
        };
        (e, loc)
      }
    };
    eprintln!("{}", e);
    print_annot(input, loc);
  }
}

use interpreter::InterpreterError;
impl InterpreterError {
  pub fn show_diagnostic(&self, input: &str) {
    // エラー情報を簡単に表示し
    eprintln!("{}", self);
    // エラー位置を指示する
    print_annot(input, self.loc.clone());
  }
}

pub fn show_trace<E: StdError>(e: E) {
  eprintln!("{}", e);
  let mut source = e.source();
  while let Some(e) = source {
    eprintln!("caused by {}", e);
    source = e.source()
  }
}

use ast::parse;
use lexer::lex;
use std::str::FromStr;
impl FromStr for Ast {
  type Err = Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let tokens = lex(s)?;
    let ast = parse(tokens)?;

    Ok(ast)
  }
}

pub mod lexer {
  use super::Annot;
  use super::Loc;

  #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
  pub enum TokenKind {
    Number(u64), //[1-9][0-9]*
    Plus,        // +
    Minus,       // -
    Asterisk,    // *
    Slash,       // /
    LParen,      // (
    RParen,      // )
  }

  use std::fmt;
  impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      use self::TokenKind::*;

      match self {
        Number(n) => n.fmt(f),
        Plus => write!(f, "+"),
        Minus => write!(f, "-"),
        Asterisk => write!(f, "*"),
        Slash => write!(f, "/"),
        LParen => write!(f, "("),
        RParen => write!(f, ")"),
      }
    }
  }

  pub type Token = Annot<TokenKind>;
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
  pub enum LexErrorKind {
    InvalidChar(char),
    Eof,
  }
  pub type LexError = Annot<LexErrorKind>;
  impl LexError {
    fn invalid_char(c: char, loc: Loc) -> Self {
      Self::new(LexErrorKind::InvalidChar(c), loc)
    }
    fn eof(loc: Loc) -> Self {
      Self::new(LexErrorKind::Eof, loc)
    }
  }

  impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      use self::LexErrorKind::*;
      let loc = &self.loc;
      match self.value {
        InvalidChar(c) => write!(f, "{}: invalid char '{}'", loc, c),
        Eof => write!(f, "End of file"),
      }
    }
  }

  pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
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
        b'0'..=b'9' => lex_a_token!(Ok(lex_number(input, pos))),
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
        b => return Err(LexError::invalid_char(b as char, Loc(pos, pos + 1))),
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
  fn lex_number(input: &[u8], mut pos: usize) -> (Token, usize) {
    use std::str::from_utf8;
    let start = pos;
    while pos < input.len() && b"1234556780".contains(&input[pos]) {
      pos += 1;
    }
    let n = from_utf8(&input[start..pos]).unwrap().parse().unwrap();
    (Token::number(n, Loc(start, pos)), pos)
  }
  fn skip_spaces(input: &[u8], mut pos: usize) -> ((), usize) {
    while pos < input.len() && b" \n\t".contains(&input[pos]) {
      pos += 1;
    }
    ((), pos)
  }

  #[test]
  fn test_lex() {
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
    );
    assert_eq!(
      lex("1 + 2 + a"),
      Err(LexError::invalid_char('a', Loc(8, 9))),
    );
  }
}

pub mod ast {
  use super::Annot;
  use super::Loc;

  #[derive(Debug, Clone, PartialEq, Eq, Hash)]
  pub enum AstKind {
    Num(u64),
    UniOp { op: UniOp, e: Box<Ast> },
    BinOp { op: BinOp, r: Box<Ast>, l: Box<Ast> },
  }
  pub type Ast = Annot<AstKind>;
  impl Ast {
    #[allow(dead_code)]
    fn num(n: u64, loc: Loc) -> Self {
      Self::new(AstKind::Num(n), loc)
    }
    fn uniop(op: UniOp, e: Ast, loc: Loc) -> Self {
      Self::new(AstKind::UniOp { op, e: Box::new(e) }, loc)
    }
    fn binop(op: BinOp, r: Ast, l: Ast, loc: Loc) -> Self {
      Self::new(
        AstKind::BinOp {
          op,
          r: Box::new(r),
          l: Box::new(l),
        },
        loc,
      )
    }
  }

  #[derive(Debug, Clone, PartialEq, Eq, Hash)]
  pub enum UniOpKind {
    Plus,
    Minus,
  }
  pub type UniOp = Annot<UniOpKind>;
  impl UniOp {
    fn plus(loc: Loc) -> Self {
      Self::new(UniOpKind::Plus, loc)
    }
    fn minus(loc: Loc) -> Self {
      Self::new(UniOpKind::Minus, loc)
    }
  }

  #[derive(Debug, Clone, PartialEq, Eq, Hash)]
  pub enum BinOpKind {
    Add,
    Sub,
    Mult,
    Div,
  }
  pub type BinOp = Annot<BinOpKind>;
  impl BinOp {
    fn add(loc: Loc) -> Self {
      Self::new(BinOpKind::Add, loc)
    }
    fn sub(loc: Loc) -> Self {
      Self::new(BinOpKind::Sub, loc)
    }
    fn mult(loc: Loc) -> Self {
      Self::new(BinOpKind::Mult, loc)
    }
    fn div(loc: Loc) -> Self {
      Self::new(BinOpKind::Div, loc)
    }
  }

  use super::lexer::Token;
  #[allow(dead_code)]
  #[derive(Debug, Clone, PartialEq, Eq, Hash)]
  pub enum ParseError {
    UnexpectedToken(Token),
    NotExpression(Token),
    NotOperator(Token),
    UnclosedOpenParen(Token),
    RedundantExpression(Token),
    Eof,
  }

  use std::fmt;
  impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      use self::ParseError::*;
      match self {
        UnexpectedToken(token) => write!(f, "{}: '{}' is not expected", token.loc, token.value),
        NotExpression(token) => write!(
          f,
          "{}: '{}' is not a start of expression",
          token.loc, token.value
        ),
        NotOperator(token) => write!(f, "{}: '{}' is not an operator", token.loc, token.value),
        UnclosedOpenParen(token) => write!(f, "{}: '{}' is not closed", token.loc, token.value),
        RedundantExpression(token) => write!(
          f,
          "{}: expression after '{}' is redundant",
          token.loc, token.value
        ),
        Eof => write!(f, "End of file"),
      }
    }
  }

  pub fn parse(tokens: Vec<Token>) -> Result<Ast, ParseError> {
    let mut tokens = tokens.into_iter().peekable();
    let ret = parse_expr(&mut tokens)?;
    match tokens.next() {
      Some(token) => Err(ParseError::RedundantExpression(token)),
      None => Ok(ret),
    }
  }

  use std::iter::Peekable;
  fn parse_expr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
  where
    Tokens: Iterator<Item = Token>,
  {
    parse_expr3(tokens)
  }

  use super::lexer::TokenKind;
  fn parse_expr3<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
  where
    Tokens: Iterator<Item = Token>,
  {
    fn parse_expr3_op<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<BinOp, ParseError>
    where
      Tokens: Iterator<Item = Token>,
    {
      let op = tokens
        .peek()
        .ok_or(ParseError::Eof)
        .and_then(|token| match token.value {
          TokenKind::Plus => Ok(BinOp::add(token.loc.clone())),
          TokenKind::Minus => Ok(BinOp::sub(token.loc.clone())),
          _ => Err(ParseError::NotOperator(token.clone())),
        })?;
      tokens.next();
      Ok(op)
    }

    parse_left_binop(tokens, parse_expr2, parse_expr3_op)
  }

  fn parse_left_binop<Tokens>(
    tokens: &mut Peekable<Tokens>,
    subexpr_parser: fn(&mut Peekable<Tokens>) -> Result<Ast, ParseError>,
    op_parser: fn(&mut Peekable<Tokens>) -> Result<BinOp, ParseError>,
  ) -> Result<Ast, ParseError>
  where
    Tokens: Iterator<Item = Token>,
  {
    let mut l = subexpr_parser(tokens)?;

    while tokens.peek().is_some() {
      let op = match op_parser(tokens) {
        Ok(op) => op,
        Err(_) => break, // no more infix op
      };
      let r = subexpr_parser(tokens)?;
      let loc = l.loc.merge(&r.loc);
      l = Ast::binop(op, l, r, loc)
    }

    Ok(l)
  }

  fn parse_expr2<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
  where
    Tokens: Iterator<Item = Token>,
  {
    let mut e = parse_expr1(tokens)?;
    loop {
      match tokens.peek().map(|token| token.value) {
        Some(TokenKind::Asterisk) | Some(TokenKind::Slash) => {
          let op = match tokens.next().unwrap() {
            Token {
              value: TokenKind::Asterisk,
              loc,
            } => BinOp::mult(loc),
            Token {
              value: TokenKind::Slash,
              loc,
            } => BinOp::div(loc),
            _ => unreachable!(),
          };

          let r = parse_expr1(tokens)?;
          let loc = e.loc.merge(&r.loc);
          e = Ast::binop(op, e, r, loc);
        }
        _ => return Ok(e),
      }
    }
  }

  fn parse_expr1<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
  where
    Tokens: Iterator<Item = Token>,
  {
    match tokens.peek().map(|token| token.value) {
      Some(TokenKind::Plus) | Some(TokenKind::Minus) => {
        let op = match tokens.next() {
          Some(Token {
            value: TokenKind::Plus,
            loc,
          }) => UniOp::plus(loc),
          Some(Token {
            value: TokenKind::Minus,
            loc,
          }) => UniOp::minus(loc),
          _ => unreachable!(),
        };
        let e = parse_atom(tokens)?;
        let loc = op.loc.merge(&e.loc);
        Ok(Ast::uniop(op, e, loc))
      }
      _ => parse_atom(tokens),
    }
  }

  fn parse_atom<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
  where
    Tokens: Iterator<Item = Token>,
  {
    tokens
      .next()
      .ok_or(ParseError::Eof)
      .and_then(|token| match token.value {
        TokenKind::Number(n) => Ok(Ast::new(AstKind::Num(n), token.loc)),
        TokenKind::LParen => {
          let e = parse_expr(tokens)?;
          match tokens.next() {
            Some(Token {
              value: TokenKind::RParen,
              loc: _,
            }) => Ok(e),
            Some(t) => Err(ParseError::RedundantExpression(t)),
            _ => Err(ParseError::UnclosedOpenParen(token)),
          }
        }
        _ => Err(ParseError::NotExpression(token)),
      })
  }

  #[test]
  fn test_parse() {
    use super::lexer::lex;
    let tokens = lex("1 + 2 * 3 - -10").unwrap();
    let ast = parse(tokens);

    assert_eq!(
      ast,
      Ok(Ast::binop(
        BinOp::sub(Loc(10, 11)),
        Ast::binop(
          BinOp::add(Loc(2, 3)),
          Ast::num(1, Loc(0, 1)),
          Ast::binop(
            BinOp::new(BinOpKind::Mult, Loc(6, 7)),
            Ast::num(2, Loc(4, 5)),
            Ast::num(3, Loc(8, 9)),
            Loc(4, 9)
          ),
          Loc(0, 9)
        ),
        Ast::uniop(
          UniOp::minus(Loc(12, 13)),
          Ast::num(10, Loc(13, 15)),
          Loc(12, 15)
        ),
        Loc(0, 15)
      ))
    )
  }
}

pub mod interpreter {
  use super::ast::*;
  use super::Annot;

  pub struct Interpreter;
  impl Interpreter {
    pub fn new() -> Self {
      Interpreter
    }

    pub fn eval(&mut self, expr: &Ast) -> Result<i64, InterpreterError> {
      match expr.value {
        AstKind::Num(n) => Ok(n as i64),
        AstKind::UniOp { ref op, ref e } => {
          let e = self.eval(e)?;
          Ok(self.eval_uniop(op, e))
        }
        AstKind::BinOp {
          ref op,
          ref l,
          ref r,
        } => {
          let l = self.eval(l)?;
          let r = self.eval(r)?;
          self
            .eval_binop(op, l, r)
            .map_err(|e| InterpreterError::new(e, expr.loc.clone()))
        }
      }
    }

    pub fn eval_uniop(&mut self, op: &UniOp, n: i64) -> i64 {
      match op.value {
        UniOpKind::Plus => n,
        UniOpKind::Minus => -n,
      }
    }

    pub fn eval_binop(&mut self, op: &BinOp, l: i64, r: i64) -> Result<i64, InterpreterErrorKind> {
      match op.value {
        BinOpKind::Add => Ok(l + r),
        BinOpKind::Sub => Ok(l - r),
        BinOpKind::Mult => Ok(l * r),
        BinOpKind::Div => {
          if r == 0 {
            Err(InterpreterErrorKind::DivisionByZero)
          } else {
            Ok(l / r)
          }
        }
      }
    }
  }

  #[derive(Debug, Clone, PartialEq, Eq, Hash)]
  pub enum InterpreterErrorKind {
    DivisionByZero,
  }
  pub type InterpreterError = Annot<InterpreterErrorKind>;

  use std::fmt;
  impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      use self::InterpreterErrorKind::*;

      match self.value {
        DivisionByZero => write!(f, "zero division error"),
      }
    }
  }
}

#[cfg(test)]
mod test {
  use super::ast::parse;
  use super::interpreter::Interpreter;
  use super::lexer::lex;

  #[test]
  fn test_interpreter() {
    let tokens = lex("1 + 2 * 3 - 10").unwrap();
    let ast = parse(tokens).unwrap();
    let mut interpreter = Interpreter::new();

    let value = interpreter.eval(&ast).unwrap();
    assert_eq!(value, 3);
  }
}
