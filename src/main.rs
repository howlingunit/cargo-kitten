use kitten::string_helpers::{first_word, to_ordinal};
use kitten::file_helpers::{file_content};
use std::{io, env};

fn main() {
  const KITTEN: &str = "🐱";
  println!("Hi {}", KITTEN);

  let mut filenames: Vec<String> = vec![].to_vec();

  for i in env::args(){
    filenames.push(i);
  }
  
  filenames.remove(0); // remove path to file

  let mut output: String = String::new();
  
  for i in filenames {
    output = output + &file_content(&i).unwrap();
    output = output + "\n";
  }
  println!("{}", output);

  println!("Bye {}", KITTEN);
}

  // old stuff

  // let mut number_of_files = filenames.len();
  // println!("default number_of_files={}", number_of_files);

  // println!("How many files do you want to open?");

  // let mut input: String = String::new();
  // match io::stdin().read_line(&mut input) {
  //   Ok(_n) => {
  //     input = first_word(input);
  //     number_of_files = input.parse().unwrap_or(1);
  //   }
  //   Err(error) => println!("Error while reading your input: {}", error),
  // }

  // println!("number_of_files={}", number_of_files);

  // let mut output = String::new();

  // for i in 1..(number_of_files + 1) {
  //   let ordinal = to_ordinal(i);
  //   input = String::from("");

  //   println!(
  //     "[{}/{}]Please enter the name/path to the {} file: ",
  //     i, number_of_files, ordinal
  //   );
  //   match io::stdin().read_line(&mut input) {
  //     Ok(_) => {
  //       input = first_word(input);
  //       println!("Opening file: {}", input); // for debugging
  //     }
  //     Err(error) => {
  //       panic!("Error while reading your input: {}", error);
  //     }
  //   }
  //   output = output + &file_content(&input).unwrap();
  // }