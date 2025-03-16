use regex::Regex;
use validator::ValidationError;

pub fn validate_alphanumunderscore(value: &str) -> Result<(), ValidationError> {
  let re = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
  if re.is_match(value) {
      return Ok(());
  } else {
      return Err(ValidationError::new("alphanumunderscore"));
  }
}
