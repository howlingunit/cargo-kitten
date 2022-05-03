use std::io; 

fn main() {
  const KITTEN: &str = "🐱";
  println!("Hi {}", KITTEN);

  let mut number_of_files: u32 = 1;
  println!("default number_of_files={}", number_of_files);

  println!("How many files do you want to open?");

  let mut input: String = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(_n) => {input.pop();}
    Err(error) => println!("Error while reading your input: {}", error),
  }
    

  number_of_files = input.parse().unwrap_or(1);
  println!("number_of_files={}", number_of_files);
  
  for i in 1..(number_of_files+1) {
    let ordinal = i.to_string()
    + match i % 10 {
      1 if i % 100 != 11 => "st",
      2 if i % 100 != 12 => "nd",
      3 if i % 100 != 13 => "rd",
      _ => "th",
    };
    println!("[{}/{}]Please enter the name/path to the {} file: ", i, number_of_files, ordinal);
    // need to ask for input here
   }

  println!("Bye {}", KITTEN);
}

