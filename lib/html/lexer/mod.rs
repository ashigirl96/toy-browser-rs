use crate::html::lexer::token::Token;
use std::iter::Peekable;
use std::str::Chars;

mod token;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.input.peek() {
            Some('<') => Token::TextToken("HOGE".to_string()),
            // TODO: consider all of words
            Some(ch) if ch.is_alphanumeric() | ch.is_whitespace() => self.consume_text(),
            None => Token::Eof,
            _ => Token::Illegal,
        };
        self.input.next();
        token
    }

    fn consume_text(&mut self) -> Token {
        let mut text = String::new();
        while let Some(ch) = &self.input.next() {
            // TODO: consider all of words
            if ch.is_alphanumeric() || ch.is_whitespace() {
                text.push_str(&ch.to_string());
            } else {
                break;
            }
        }
        Token::TextToken(text)
    }

    fn skip_whitespace(&mut self) {
        while self.input.next_if(|&x| x.is_whitespace()).is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use crate::html::lexer::token::Token;
    use crate::html::lexer::Lexer;

    #[test]
    fn test_eof() {
        let input = "";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_next_token() {
        let input = r#"Hello world

はろーわーるど"#;
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::TextToken(input.to_string()));
    }
}
