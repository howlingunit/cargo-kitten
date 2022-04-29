use std::io; 

fn main() {
  const KITTEN: &str = "🐱";
  println!("Hi {}", KITTEN);

  let mut number_of_files: u32 = 1;
  println!("default number_of_files={}", number_of_files);

  println!("How many files do you want to open?");

  let mut input: String = String::new();
  // code will not compile until you declare ‘input’ here
  match io::stdin().read_line(&mut input) {
    Ok(_n) => {}
    Err(error) => println!("Error while reading your input: {}", error),
  }

  input.pop();  

  number_of_files = input.parse().unwrap_or(1);
  let n = number_of_files;
  println!("updated number_of_files={}", number_of_files);

  while number_of_files != 0 {
    println!("Please enter the name/path to a file: ");
    // need to ask for input here
 
    number_of_files -= 1; // or: number_of_files = number_of_files - 1;
  }

  println!("Bye {}", KITTEN);
}

