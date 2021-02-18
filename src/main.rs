mod calculator;
mod calculator_ref;

mod draw_functions;
pub use crate::draw_functions::functions as funcs;
pub use crate::draw_functions::renderers as render;

#[allow(dead_code)]
fn render() -> Result<(), Box<dyn std::error::Error>> {
  let path = "image.svg";
  let function = funcs::sin_rr;
  let canvas = render::Canvas::default();

  render::draw(path, function, canvas)?;

  Ok(())
}

use std::io;
fn prompt(s: &str) -> io::Result<()> {
  use std::io::{stdout, Write};

  let stdout = stdout();
  let mut stdout = stdout.lock();
  stdout.write_all(s.as_bytes())?;
  stdout.flush()
}

fn main() {
  use std::io::{stdin, BufRead, BufReader};

  let stdin = stdin();
  let stdin = stdin.lock();
  let stdin = BufReader::new(stdin);
  let mut lines = stdin.lines();

  loop {
    pub use crate::calculator_ref::ast;
    use std::error::Error;

    prompt("> ").unwrap();
    if let Some(Ok(line)) = lines.next() {
      let ast = match line.parse::<ast::Ast>() {
        Ok(ast) => ast,
        Err(e) => {
          eprintln!("{}", e);
          let mut source = e.source();
          while let Some(e) = source {
            eprintln!("caused by {}", e);
            source = e.source()
          }
          continue;
        }
      };
      // let token = lexer::lex(&line);
      println!("{:?}", ast);
    } else {
      break;
    }
  }
}
