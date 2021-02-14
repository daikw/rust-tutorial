pub mod functions {
  extern crate libm;

  extern crate rand;
  use rand::{thread_rng, Rng};

  pub type Function = fn(f64, f64) -> f64;
  pub type F = Function; // alias

  fn sampler(len: usize) -> usize {
    let mut rng = thread_rng();
    let rand = rng.gen::<f64>(); // range == (0, 1)

    (rand * len as f64).floor() as usize
  }

  pub fn sample() -> F {
    let funcs = [sum, prod, sin_rr];
    let index = sampler(funcs.len());

    funcs[index]
  }

  pub fn sum(x: f64, y: f64) -> f64 {
    x + y
  }

  pub fn prod(x: f64, y: f64) -> f64 {
    x * y
  }

  pub fn sin_rr(x: f64, y: f64) -> f64 {
    let r = libm::hypot(x, y);
    r.sin() / r
  }

  #[cfg(test)]
  mod test {
    use super::*;

    #[test]
    fn test_sum() {
      assert_eq!(sum(2.0, 2.0), 4.0);
      assert_ne!(sum(1.0, 2.0), 10.0);
    }

    #[test]
    fn test_prod() {
      assert_eq!(prod(2.0, 2.0), 4.0);
      assert_eq!(prod(0.0, 2.0), 0.0);
      assert_eq!(prod(2.5, 200.0), 500.0);
    }

    #[test]
    fn test_sin_rr() {
      assert!(sin_rr(2.0, 2.0) - 4.0 <= 0.000_01);
    }

    #[test]
    fn test_sampler() {
      let length = 5;
      let test_count = 20;

      for _ in 1..test_count {
        let index = sampler(length);
        println!("{:?}", index); // print with `cargo test -- --nocapture`
        assert!(index < length);
      }
    }
  }
}

pub mod renderers {
  use crate::draw_functions::functions;

  pub struct Canvas {
    width: i64,
    height: i64,
    xyrange: f64,
    cells: i64,
  }

  impl Canvas {
    pub fn default() -> Canvas {
      Canvas {
        width: 600,
        height: 320,
        xyrange: 30.0,
        cells: 100,
      }
    }

    pub fn new(width: i64, height: i64, xyrange: f64, cells: i64) -> Canvas {
      Canvas {
        width,
        height,
        xyrange,
        cells,
      }
    }

    fn project(&self, f: functions::F, i: i64, j: i64) -> [f64; 2] {
      let x = self.xyrange * ((i as f64) / (self.cells as f64) - 0.5);
      let y = self.xyrange * ((j as f64) / (self.cells as f64) - 0.5);
      let z = f(x, y);
      let angle = std::f64::consts::FRAC_PI_6;
      let xyscale = (self.width as f64) / 2.0 / self.xyrange;
      let zscale = (self.height as f64) * 0.4;
      let sx = (self.width as f64) / 2.0 + (x - y) * angle.cos() * xyscale;
      let sy = (self.height as f64) / 2.0 + (x + y) * angle.sin() * xyscale - z * zscale;

      [sx, sy]
    }
  }

  use std::fs::File;
  use std::io::{BufWriter, Write};
  use std::path::Path;

  struct SvgRenderer {
    f: functions::F,
    canvas: Canvas,
  }

  impl SvgRenderer {
    pub fn new(function: functions::F, canvas: Canvas) -> SvgRenderer {
      SvgRenderer {
        f: function,
        canvas,
      }
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
      writer.write_all(header.as_bytes())?;

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
          writer.write_all(polygon.as_bytes())?;
        }
      }
      let footer = "</svg>";
      writer.write_all(footer.as_bytes())?;

      Ok(())
    }
  }

  pub fn draw(
    path_string: &str,
    f: functions::F,
    canvas: Canvas,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let renderer = SvgRenderer::new(f, canvas);
    renderer.write(path_string)?;

    Ok(())
  }
}
