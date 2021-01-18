mod draw_functions;
mod guess_the_number;
mod primitives;

pub use crate::guess_the_number::dialogue;
pub use crate::primitives::functions;
pub use crate::primitives::variables;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // fn main() {
  // functions::puts();
  // dialogue::main();
  // variables::example();

  let path = "image.svg";
  let function = draw_functions::f;
  draw_functions::draw(path, function)?;

  Ok(())
}
