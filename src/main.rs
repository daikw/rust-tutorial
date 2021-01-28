mod draw_functions;

pub use crate::draw_functions::functions as funcs;
pub use crate::draw_functions::renderers as render;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let path = "image.svg";
  let function = funcs::sin_rr;
  let canvas = render::Canvas::default();

  render::draw(path, function, canvas)?;

  Ok(())
}
