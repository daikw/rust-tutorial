type Function = fn(f64, f64) -> f64;
type F = Function; // alias

pub fn f(x: f64, y: f64) -> f64 {
  return x + y;
}

extern crate libm;
pub fn sin_rr(x: f64, y: f64) -> f64 {
  let r = libm::hypot(x, y);
  return r.sin() / r;
}

pub struct Canvas {
  width: i64,
  height: i64,
  xyrange: f64,
  cells: i64,
}

impl Canvas {
  pub fn default() -> Canvas {
    return Canvas {
      width: 600,
      height: 320,
      xyrange: 30.0,
      cells: 100,
    };
  }

  pub fn new(width: i64, height: i64, xyrange: f64, cells: i64) -> Canvas {
    return Canvas {
      width: width,
      height: height,
      xyrange: xyrange,
      cells: cells,
    };
  }

  pub fn project(&self, f: F, i: i64, j: i64) -> [f64; 2] {
    let x = self.xyrange * ((i as f64) / (self.cells as f64) - 0.5);
    let y = self.xyrange * ((j as f64) / (self.cells as f64) - 0.5);
    let z = f(x, y);

    let angle = std::f64::consts::FRAC_PI_6;
    let xyscale = (self.width as f64) / 2.0 / self.xyrange;
    let zscale = (self.height as f64) * 0.4;

    let sx = (self.width as f64) / 2.0 + (x - y) * angle.cos() * xyscale;
    let sy = (self.height as f64) / 2.0 + (x + y) * angle.sin() * xyscale - z * zscale;

    return [sx, sy];
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

    let header = format!(
      "<?xml version=\"1.0\"?><svg xmlns=\"http://www.w3.org/2000/svg\"
         style='stroke: grey; fill: white; stroke-width: 0.7' width='{}' height='{}'>\n",
      self.canvas.width, self.canvas.height
    );
    writer.write(header.as_bytes())?;

    for i in 0..self.canvas.cells {
      for j in 0..self.canvas.cells {
        let [ax, ay] = self.canvas.project(self.f, i + 1, j);
        let [bx, by] = self.canvas.project(self.f, i, j);
        let [cx, cy] = self.canvas.project(self.f, i, j + 1);
        let [dx, dy] = self.canvas.project(self.f, i + 1, j + 1);

        let polygon = format!(
          "  <polygon points='{},{} {},{} {},{} {},{}'/>\n",
          ax, ay, bx, by, cx, cy, dx, dy
        );

        writer.write(polygon.as_bytes())?;
      }
    }

    let footer = "</svg>";
    writer.write(footer.as_bytes())?;

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
