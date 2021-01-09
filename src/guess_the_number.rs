extern crate rand;

pub mod dialogue {
  use rand::Rng;
  use std::cmp::Ordering;
  use std::io;

  pub fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
      println!("Input your number ...");
      let mut guess = String::new();

      io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read_line");

      let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
      };

      println!("You guessed: {}", guess);

      match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too large"),
        Ordering::Equal => {
          println!("You win!");
          break;
        }
      }
    }
  }
}
