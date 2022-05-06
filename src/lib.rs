pub mod string_helpers{
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