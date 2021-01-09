mod guess_the_number;
mod primitives;

pub use crate::guess_the_number::dialogue;
pub use crate::primitives::functions;

fn main() {
  println!("Hello, world!");

  functions::puts();

  dialogue::main();
}
