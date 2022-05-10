use kitten::string_helpers::{first_word, to_ordinal};
use kitten::file_helpers::{file_content};
use std::{io, env};

fn main() {
  const KITTEN: &str = "🐱";
  println!("Hi {}", KITTEN);

  let mut filenames: Vec<String>;

  for i in env::args(){
    filenames.push(i);
  }

  let mut number_of_files: u32 = 1;
  println!("default number_of_files={}", number_of_files);

  println!("How many files do you want to open?");

  let mut input: String = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(_n) => {
      input = first_word(input);
      number_of_files = input.parse().unwrap_or(1);
    }
    Err(error) => println!("Error while reading your input: {}", error),
  }

  println!("number_of_files={}", number_of_files);

  let mut output = String::new();

  for i in 1..(number_of_files + 1) {
    let ordinal = to_ordinal(i);
    input = String::from("");

    println!(
      "[{}/{}]Please enter the name/path to the {} file: ",
      i, number_of_files, ordinal
    );
    match io::stdin().read_line(&mut input) {
      Ok(_) => {
        input = first_word(input);
        println!("Opening file: {}", input); // for debugging
      }
      Err(error) => {
        panic!("Error while reading your input: {}", error);
      }
    }
    output = output + &file_content(&input).unwrap();
  }
  println!("{}", output);

  println!("Bye {}", KITTEN);
}
