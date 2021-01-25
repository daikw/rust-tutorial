type Function = fn(i64, i64) -> i64;
type F = Function; // alias

pub fn f(x: i64, y: i64) -> i64 {
  return x + y;
}

struct SVGRenderer {
  f: Function,
}

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

impl SVGRenderer {
  pub fn new(function: F) -> SVGRenderer {
    return SVGRenderer { f: function };
  }

  fn write(&self, path_string: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(path_string);
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    let z = (self.f)(100, 50);
    let text = format!("{}, {}, {}", "<svg>", z, "</svg>");

    writer.write(text.as_bytes())?;
    Ok(())
  }
}

pub fn draw(path_string: &str, f: Function) -> Result<(), Box<dyn std::error::Error>> {
  let renderer = SVGRenderer::new(f);
  renderer.write(path_string)?;
  Ok(())
}
