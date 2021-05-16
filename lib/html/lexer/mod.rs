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
        self.input.next(); // skip `<`
        match self.input.peek() {
            Some(ch) if ch.is_alphanumeric() => {
                let tag_name = self.expect_tag_name();
                let attributes = match self.input.peek() {
                    Some('>') => {
                        self.input.next();
                        Attributes::new()
                    }
                    Some('a'..='z' | 'A'..='Z') => self.expect_attributes(),
                    _ => {
                        panic!("ERROR");
                    }
                };
                Token::StartTagToken(ElementData::new(tag_name, attributes))
            }
            Some('>') => {
                self.input.next(); // skip `>`
                Token::StartTagToken(ElementData::new(String::from("div"), Attributes::new()))
            }
            Some('/') => {
                self.input.next(); // skip `/`
                let tag_name = self.expect_tag_name();
                match self.input.peek() {
                    Some('>') => self.input.next(),
                    _ => None,
                };
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
        if tag_name.is_empty() {
            return String::from("div");
        }
        tag_name
    }

    fn expect_attributes(&mut self) -> Attributes {
        // id="names" class="table"
        let mut attributes = Attributes::new();
        loop {
            let (key, value) = match self.input.peek() {
                Some('>') => {
                    self.input.next();
                    break;
                }
                Some(_) => self.expect_attribute(),
                None => panic!("Cannot parse token in expect_attributes"),
            };
            attributes.insert(key, value);
        }
        attributes
    }

    fn expect_attribute(&mut self) -> (String, String) {
        // e.g. class="table"
        let key = self.consume(&|x| x.is_ascii_alphabetic());
        self.skip_next_ch(&'=');
        self.skip_next_ch(&'"');
        let value = self.consume(&|x| x != &'"');
        self.skip_next_ch(&'"');
        self.skip_whitespace();
        (key, value)
    }

    fn consume_text(&mut self) -> Token {
        let text = self.consume(&|ch| ch.is_alphanumeric() || ch.is_whitespace());
        Token::TextToken(text)
    }

    fn skip_next_ch(&mut self, ch: &char) {
        match self.input.peek() {
            Some(c) if c == ch => self.input.next(),
            _ => panic!("cannot found {}", ch),
        };
    }

    fn consume<F>(&mut self, consume_condition: &F) -> String
    where
        F: Fn(&char) -> bool,
    {
        let mut s = String::new();
        while let Some(ch) = self.input.next_if(consume_condition) {
            s.push_str(&ch.to_string());
        }
        s
    }

    fn skip_whitespace(&mut self) {
        while self.input.next_if(|&x| x.is_whitespace()).is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use crate::html::lexer::token::{Attributes, ElementData, Token};
    use crate::html::lexer::Lexer;

    fn from_vec(attributes: Vec<(String, String)>) -> Attributes {
        attributes.iter().cloned().collect()
    }

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
    fn test_consume_start_tag() {
        let input = r#"
<>
<div  className="table"  id="names">
<a href="https://example.com">
"#;
        let mut lexer = Lexer::new(input);
        let attr = Attributes::new();
        let expects = vec![
            Token::StartTagToken(ElementData::new("div".to_string(), attr)),
            Token::StartTagToken(ElementData::new(
                "div".to_string(),
                from_vec(vec![
                    ("className".to_string(), "table".to_string()),
                    ("id".to_string(), "names".to_string()),
                ]),
            )),
            Token::StartTagToken(ElementData::new(
                "a".to_string(),
                from_vec(vec![(
                    "href".to_string(),
                    "https://example.com".to_string(),
                )]),
            )),
        ];
        for expect in expects {
            let token = lexer.next_token();
            assert_eq!(token, expect);
        }
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
