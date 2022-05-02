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
    let fomattedNumb: String = i.to_string();
    match fomattedNumb {

    }
    println!("[{}/{}]Please enter the name/path to a file number {}: ", i, number_of_files, i);
    // need to ask for input here
   }

  println!("Bye {}", KITTEN);
}

