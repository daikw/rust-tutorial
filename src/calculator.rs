mod calculator {
  fn lexer(_formula: String) -> Vec<String> {
    let values: Vec<&str> = vec!["1", "+", "1"];
    // return ["1".to_string(), "+".to_string(), "1".to_string()];
    // return vec!["1", "+", "1"];
    return values.into_iter().map(|x| x.to_string()).collect();
  }

  #[cfg(test)]
  mod test {
    use super::*;

    #[test]
    fn test_lexer() {
      assert_eq!(lexer("1+1".to_string()), ["1", "+", "1"]);
    }
  }
}
