use std::{iter::Peekable, str::CharIndices};

use crate::token::Token;

pub(crate) struct Tokenizer<'a> {
    str: &'a str,
    chars: Peekable<CharIndices<'a>>,
    skip_comments: bool,
}

impl<'a> Tokenizer<'a> {
    #[allow(dead_code)]
    pub fn new(str: &'a str) -> Self {
        Self {
            str,
            chars: str.char_indices().peekable(),
            skip_comments: false,
        }
    }

    /// Skip comment tokens when iterating.
    pub fn skip_comments(&mut self) {
        self.skip_comments = true;
    }

    fn rewind_until(&mut self, chars: &[char]) -> usize {
        let mut offset = 0;

        loop {
            // Peek next char
            let Some((_, ch)) = self.chars.peek() else {
                break;
            };

            if chars.contains(ch) {
                break;
            }

            // Take next char
            if let Some((pos, _)) = self.chars.next() {
                offset = pos;
            }
        }

        offset
    }

    fn token(&self, start: usize, end: usize) -> Token<'a> {
        Token::new(&self.str[start..end])
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some((start, ch)) = self.chars.next() else {
                return None;
            };

            let token = match ch {
                '[' | ']' => self.token(start, start + 1),
                ' ' | '\n' | '\t' | '\r' => continue,
                '"' => {
                    let mut end = self.rewind_until(&['"']);

                    // Consume remaining "
                    if let Some((pos, _)) = self.chars.next() {
                        end = pos;
                    }

                    self.token(start, end + 1)
                }
                '#' => {
                    let end = self.rewind_until(&['\r', '\n']);

                    if self.skip_comments {
                        continue;
                    }

                    self.token(start, end + 1)
                }
                _ => {
                    let mut end = self.rewind_until(&[' ', '\r', '\n', '\t', '"', '[', ']']);
                    if end == 0 {
                        end = start;
                    }

                    self.token(start, end + 1)
                }
            };

            return Some(token);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_line() {
        let mut t = Tokenizer::new("");

        assert_eq!(t.next(), None);
        assert_eq!(t.next(), None);
    }

    #[test]
    fn single_token() {
        let mut t = Tokenizer::new("Scale");

        assert_eq!(t.next(), Some(Token::new("Scale")));
        assert_eq!(t.next(), None);
    }

    #[test]
    fn two_tokens() {
        let mut t = Tokenizer::new("Scale Scale");

        assert_eq!(t.next(), Some(Token::new("Scale")));
        assert_eq!(t.next(), Some(Token::new("Scale")));
        assert_eq!(t.next(), None);
    }

    #[test]
    fn skip_newlines() {
        let str = r#"


        "#;

        let mut t = Tokenizer::new(str);
        assert_eq!(t.next(), None);
    }

    #[test]
    fn brackets() {
        let mut t = Tokenizer::new("[ abc ]");

        assert_eq!(t.next(), Some(Token::new("[")));
        assert_eq!(t.next(), Some(Token::new("abc")));
        assert_eq!(t.next(), Some(Token::new("]")));
        assert_eq!(t.next(), None);
    }

    #[test]
    fn comment_start() {
        let str = r#"
# Comment

Scale

"#;

        let mut t = Tokenizer::new(str);

        assert_eq!(t.next(), Some(Token::new("# Comment")));
        assert_eq!(t.next(), Some(Token::new("Scale")));
    }

    #[test]
    fn skip_comments() {
        let str = r#"
# Comment

Scale

"#;

        let mut t = Tokenizer::new(str);
        t.skip_comments();

        assert_eq!(t.next(), Some(Token::new("Scale")));
        assert_eq!(t.next(), None);
    }

    #[test]
    fn comment_middle() {
        let str = r#"
Scale

# Comment"#;

        let mut t = Tokenizer::new(str);

        assert_eq!(t.next(), Some(Token::new("Scale")));
        assert_eq!(t.next(), Some(Token::new("# Comment")));

        assert_eq!(t.next(), None);
    }

    #[test]
    fn quotes_single() {
        let mut t = Tokenizer::new(r#" "test" "#);

        assert_eq!(t.next(), Some(Token::new("\"test\"")));
        assert_eq!(t.next(), None);
    }

    #[test]
    fn quotes_two() {
        let mut t = Tokenizer::new(r#" "foo" [] "bar" "#);

        assert_eq!(t.next(), Some(Token::new("\"foo\"")));

        assert_eq!(t.next(), Some(Token::new("[")));
        assert_eq!(t.next(), Some(Token::new("]")));

        assert_eq!(t.next(), Some(Token::new("\"bar\"")));

        assert_eq!(t.next(), None);
    }

    #[test]
    fn single_quote() {
        let mut t = Tokenizer::new("foo \"abc");

        assert_eq!(t.next(), Some(Token::new("foo")));
        assert_eq!(t.next(), Some(Token::new("\"abc")));

        assert_eq!(t.next(), None);
    }

    #[test]
    fn single_quote_with_spaces() {
        let mut t = Tokenizer::new("foo \"abc test [] ");

        assert_eq!(t.next(), Some(Token::new("foo")));
        assert_eq!(t.next(), Some(Token::new("\"abc test [] ")));

        assert_eq!(t.next(), None);
    }

    #[test]
    fn just_quote() {
        let mut t = Tokenizer::new("\"");

        assert_eq!(t.next(), Some(Token::new("\"")));
        assert_eq!(t.next(), None);
    }

    #[test]
    fn parse_scale() {
        let mut t = Tokenizer::new("Scale -1 1 1");

        assert_eq!(t.next(), Some(Token::new("Scale")));
        assert_eq!(t.next(), Some(Token::new("-1")));
        assert_eq!(t.next(), Some(Token::new("1")));
        assert_eq!(t.next(), Some(Token::new("1")));

        assert_eq!(t.next(), None);
    }
}
