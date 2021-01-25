type Function = fn(i64, i64) -> i64;
type F = Function; // alias

pub fn f(x: i64, y: i64) -> i64 {
  return x + y;
}

pub struct Canvas {
  xrange: [i64; 2],
  yrange: [i64; 2],
}

impl Canvas {
  pub fn default() -> Canvas {
    return Canvas {
      xrange: [-100, 100],
      yrange: [-100, 100],
    };
  }

  pub fn new(xrange: [i64; 2], yrange: [i64; 2]) -> Canvas {
    return Canvas {
      xrange: xrange,
      yrange: yrange,
    };
  }
}

struct SVGRenderer {
  f: Function,
  canvas: Canvas,
}

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

impl SVGRenderer {
  pub fn new(function: F, canvas: Canvas) -> SVGRenderer {
    return SVGRenderer {
      f: function,
      canvas: canvas,
    };
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

pub fn draw(
  path_string: &str,
  f: Function,
  canvas: Canvas,
) -> Result<(), Box<dyn std::error::Error>> {
  let renderer = SVGRenderer::new(f, canvas);
  renderer.write(path_string)?;
  Ok(())
}
