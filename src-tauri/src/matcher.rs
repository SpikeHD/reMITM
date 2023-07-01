/**
 * Handle wildcards
 */
pub fn _string_matches_wildcard(string: &str, wildcard: &str) -> bool {
  let mut string_chars = string.chars();
  let mut wildcard_chars = wildcard.chars();

  loop {
    match (string_chars.next(), wildcard_chars.next()) {
      (Some(string_char), Some(wildcard_char)) => {
        if string_char != wildcard_char && wildcard_char != '*' {
          return false;
        }
      }
      (Some(_), None) => return false,
      (None, Some(wildcard_char)) => {
        if wildcard_char != '*' {
          return false;
        }
      }
      (None, None) => return true,
    }
  }
}
