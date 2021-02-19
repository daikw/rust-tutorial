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

  let mut interpreter = calculator_ref::interpreter::Interpreter::new();

  let stdin = stdin();
  let stdin = stdin.lock();
  let stdin = BufReader::new(stdin);
  let mut lines = stdin.lines();

  loop {
    pub use crate::calculator_ref::ast;
    use calculator_ref::show_trace;

    prompt("> ").unwrap();
    if let Some(Ok(line)) = lines.next() {
      let ast = match line.parse::<ast::Ast>() {
        Ok(ast) => ast,
        Err(e) => {
          e.show_diagnostic(&line);
          show_trace(e);
          continue;
        }
      };
      let n = match interpreter.eval(&ast) {
        Ok(n) => n,
        Err(e) => {
          e.show_diagnostic(&line);
          show_trace(e);
          continue;
        }
      };

      println!("{:?}", n);
    } else {
      break;
    }
  }
}
