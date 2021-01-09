pub mod functions {
  pub fn puts() {
    println!("called primitives function");
  }
}

pub mod variables {
  pub fn example() {
    println!("this is valid:");
    println!(
      "\n\
      ```\n\
      let spaces = \"    \";\n\
      let spaces = spaces.len();\n\
      ```\n\
      "
    );

    let spaces = "    ";
    let spaces = spaces.len();

    println!("result: {}\n", spaces);

    println!("but this is invalid:");
    println!(
      "\n\
      ```\n\
      let mut spaces = \"    \";\n\
      spaces = spaces.len();\n\
      ```\n\
      "
    );
  }
}
