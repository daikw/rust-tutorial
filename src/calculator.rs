mod components {
  #[allow(dead_code)]
  fn lexer(formula: String) -> Vec<String> {
    formula.chars().map(|x| x.to_string()).collect()
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
