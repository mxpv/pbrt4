#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Token<'a> {
    str: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(str: &'a str) -> Self {
        Token { str }
    }

    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool {
        // Empty tokens are not allowed, something wrong with tokenizer
        if self.str.is_empty() {
            return false;
        }

        // Validate quoted string

        let starts_with_quote = self.str.starts_with('\"');
        let ends_with_quote = self.str.ends_with('\"');

        if starts_with_quote || ends_with_quote {
            // Should both start and end with "
            if starts_with_quote != ends_with_quote {
                return false;
            }

            if self.str.len() < 2 {
                return false;
            }
        }

        // No spaces unless its a quotes string
        if !starts_with_quote && self.str.contains(' ') {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Token;

    #[test]
    fn is_valid_token() {
        assert!(Token::new("bar").is_valid());
        assert!(Token::new("\"foo\"").is_valid());
        assert!(Token::new("\"foo bar\"").is_valid());

        assert!(!Token::new("").is_valid());
        assert!(!Token::new("\"").is_valid());
        assert!(!Token::new("\"foo").is_valid());
        assert!(!Token::new("bar\"").is_valid());
        assert!(!Token::new("foo bar").is_valid());
    }
}
