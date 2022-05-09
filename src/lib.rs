pub mod string_helpers {
  /// Use this function to get the string ordinal of a number
  /// ```
  /// use kitten::string_helpers::to_ordinal;
  /// let test = to_ordinal(5);
  /// assert_eq!(test, "5th");
  pub fn to_ordinal(number: u32) -> String {
    let mut ordinal = number.to_string();
    match number % 10 {
      1 if number % 100 != 11 => ordinal.push_str("st"),
      2 if number % 100 != 12 => ordinal.push_str("nd"),
      3 if number % 100 != 13 => ordinal.push_str("rd"),
      _ => ordinal.push_str("th"),
    };

    return ordinal;
  }

  /// Use this function to get the first word out of a string
  /// the input is a string and it splits by whitespace and returns the first value
  /// ```
  /// use kitten::string_helpers::first_word;
  /// let message = first_word("some text".to_string());
  /// assert_eq!(message, "some");
  /// ```
  pub fn first_word(input: String) -> String {
    let split: Vec<&str> = input.split_whitespace().collect();
    return split[0].to_string();
  }
}

#[cfg(test)]
mod string_helpers_tests {
  use super::string_helpers;

  #[test]
  fn to_ordinal_th() {
    let cases: Vec<(u32, &str)> = vec![
      (10, "10th"),
      (111, "111th"),
      (0, "0th"),
      (999, "999th"),
      (250, "250th"),
    ];
    for case in cases {
      to_ordinal_test(case.0, String::from(case.1));
    }
  }

  #[test]
  fn to_ordinal_not_th() {
    let cases: Vec<(u32, &str)> = vec![
      (1, "1st"),
      (531, "531st"),
      (2, "2nd"),
      (132, "132nd"),
      (3, "3rd"),
      (153, "153rd"),
    ];

    for case in cases {
      to_ordinal_test(case.0, String::from(case.1));
    }
  }

  fn to_ordinal_test(number: u32, expected_answer: String) {
    let answer = string_helpers::to_ordinal(number);
    assert_eq!(expected_answer, answer);
  }

  #[test]
  fn first_word_test() {
    let cases: Vec<(&str, &str)> = vec![
      ("mutiple words", "mutiple"),
      ("a lot of words in a string", "a"),
    ];

    for case in cases {
      let answer = string_helpers::first_word(case.0.to_string());
      assert_eq!(answer, case.1.to_string());
    }
  }
}

pub mod file_helpers {
  use std::fs;

  pub fn file_content(filename: &str) -> Result<String, String> {
    match fs::read_to_string(filename) {
      Ok(Content) => Ok(Content),
      Err(error) if format!("{}", error).contains("No such file") => {
        Err(format!("{} does not exist", filename))
      }
      Err(_) => Err(format!("Unknown error opening {}", filename)),
    }
  }
}

#[cfg(test)]
mod file_helpers_tests {
  use super::file_helpers;

  #[test]
  fn file_content_file_exists() {
    todo!();
  }

  #[test]
  fn file_content_file_not_exists() {
    let filename: &str = "";
    assert!(file_helpers::file_content(filename).is_err());
  }
}
