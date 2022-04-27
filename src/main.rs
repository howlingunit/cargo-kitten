use std::io; // import io module

fn main() {
  const KITTEN: &str = "🐱";
  println!("Hi {}", KITTEN);

  let number_of_files = 1;
  println!("{} file(s) need to be opened.", number_of_files);

  println!("How many files do you want to open?");

  // code will not compile until you declare ‘input’ here
  match io::stdin().read_line(&mut input) {
    Ok(n) => {
      println!("{} bytes read", n);
      println!("Your input was: ({})", input); // note the parentheses
    }
    Err(error) => println!("Error while reading your input: {}", error),
  }

  println!("Bye {}", KITTEN);
}
