type Function = fn(i64, i64) -> i64;

pub fn f(x: i64, y: i64) -> i64 {
  return x + y;
}

use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::path::Path;

fn write<R: Read>(path_string: &str, buf: R) -> Result<(), Box<dyn std::error::Error>> {
  let path = Path::new(path_string);
  let mut file = File::create(path)?;
  for result in BufReader::new(buf).bytes() {
    let byte = result?;
    file.write_all(&[byte])?;
  }
  file.flush()?;

  Ok(())
}

pub fn render(f: Function) -> impl Read {
  let z = f(100, 50);
  format!("{}, {}, {}", "<svg>", z, "</svg>");

  return io::stdin();
}

pub fn draw(path_string: &str) -> Result<(), Box<dyn std::error::Error>> {
  let renderer = render(f);
  write(path_string, renderer)
}
