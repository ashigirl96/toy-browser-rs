use crate::html::lexer::token::{Attributes, ElementData, Token};
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
            Some('<') => self.consume_tag(),
            // TODO: consider all of words
            Some(ch) if ch.is_alphanumeric() => self.consume_text(),
            None => Token::Eof,
            _ => Token::Illegal,
        };
        token
    }

    fn consume_tag(&mut self) -> Token {
        self.input.next();
        match self.input.next() {
            Some('>') => {
                Token::StartTagToken(ElementData::new(String::from("div"), Attributes::new()))
            }
            Some('/') => {
                let tag_name = self.expect_tag_name();
                Token::EndTagToken(tag_name)
            }
            Some(_) => Token::EndTagToken("hoge".to_string()),
            None => panic!("cannot parse token"),
        }
    }

    fn expect_tag_name(&mut self) -> String {
        let mut tag_name = String::new();
        while let Some(ch) = &self.input.next() {
            if ch.is_alphabetic() {
                tag_name.push_str(&&ch.to_string());
            } else {
                break;
            }
        }
        self.skip_whitespace();
        match self.input.peek() {
            Some('>') => self.input.next(),
            _ => None,
        };
        if tag_name.is_empty() {
            return String::from("div");
        }
        tag_name
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
    use crate::html::lexer::token::{ElementData, Token};
    use crate::html::lexer::Lexer;

    #[test]
    fn test_eof() {
        let input = "";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_consume_text() {
        let input = r#"Hello world

はろーわーるど"#;
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::TextToken(input.to_string()));
    }

    #[test]
    fn test_consume_end_tag() {
        let input = r#"
</>
</div >
</div>
"#;
        let mut lexer = Lexer::new(input);
        let expects = vec![
            Token::EndTagToken("div".to_string()),
            Token::EndTagToken("div".to_string()),
            Token::EndTagToken("div".to_string()),
        ];
        for expect in expects {
            let token = lexer.next_token();
            assert_eq!(token, expect);
        }
    }
}
