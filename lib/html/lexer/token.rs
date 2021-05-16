use std::collections::BTreeMap;
use std::fmt;

#[allow(dead_code)]
pub enum Token {
    TextToken(String),
    StartTagToken(ElementData),
    EndTagToken(String),
    SelfClosingTagToken(ElementData),
    CommentToken(String),
    // ErrorToken, TODO: i'll implement if i feel like it.
    // DoctypeToken(String), TODO: i'll implement if i feel like it.
}

pub struct ElementData {
    tag_name: String,
    attributes: Attributes,
}

type Attributes = BTreeMap<String, String>;

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Token::TextToken(ref s) => write!(f, "{}", s),
            Token::StartTagToken(ref element) => {
                write!(f, "<{}>", Self::element_to_string(element))
            }
            Token::EndTagToken(ref tag_name) => write!(f, "</{}>", tag_name),
            Token::SelfClosingTagToken(ref element) => {
                write!(f, "<{} />", Self::element_to_string(element))
            }
            Token::CommentToken(ref s) => write!(f, "<!-- {} -->", s),
        }
    }
}

impl Token {
    fn element_to_string(element: &ElementData) -> String {
        if element.attributes.is_empty() {
            return element.tag_name.clone();
        }
        let mut s = String::from("");
        s.push_str(&element.tag_name.clone());
        for (attr, value) in element.attributes.iter() {
            s.push_str(&format!(r#" {}="{}""#, attr, value));
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use crate::html::lexer::token::{Attributes, ElementData, Token};

    #[test]
    fn test_text_token() {
        let token = Token::TextToken("Hello, world".to_string());
        assert_eq!(format!("{:?}", token), "Hello, world".to_string());
    }

    #[test]
    fn test_start_tag_token() {
        let tag_name = String::from("div");
        let mut attributes = Attributes::new();
        attributes.insert("id".to_string(), "names".to_string());
        attributes.insert("className".to_string(), "table".to_string());
        let token = Token::StartTagToken(ElementData {
            tag_name,
            attributes,
        });
        assert_eq!(
            format!("{:?}", token),
            r#"<div className="table" id="names">"#
        );
    }

    #[test]
    fn test_end_tag_token() {
        let token = Token::EndTagToken("div".to_string());
        assert_eq!(format!("{:?}", token), "</div>".to_string());
    }

    #[test]
    fn test_self_closing_tag_token() {
        let tag_name = String::from("a");
        let mut attributes = Attributes::new();
        attributes.insert("href".to_string(), "https://example.com".to_string());
        let token = Token::SelfClosingTagToken(ElementData {
            tag_name,
            attributes,
        });
        assert_eq!(
            format!("{:?}", token),
            r#"<a href="https://example.com" />"#.to_string()
        );
    }

    #[test]
    fn test_comment() {
        let token = Token::CommentToken("TODO: implement table".to_string());
        assert_eq!(
            format!("{:?}", token),
            "<!-- TODO: implement table -->".to_string()
        );
    }
}
