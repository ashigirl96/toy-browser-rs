use crate::html::lexer::token::{ElementData, Token};
use anyhow::Result;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
    // Document,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    node_type: NodeType,
    children: Vec<Node>,
}

impl Node {
    pub fn new(node_type: NodeType) -> Self {
        Self {
            node_type,
            children: vec![],
        }
    }
}

type Dom = Vec<Node>;

#[derive(Debug)]
pub struct DOMParser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
}

impl<'a> DOMParser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens: tokens.iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Dom> {
        let mut dom: Dom = vec![];
        while !self.next_token_is(&Token::Eof) {
            dom.push(self.parse_node()?);
        }
        Ok(dom)
    }

    fn parse_node(&mut self) -> Result<Node> {
        let node = match self.tokens.next() {
            Some(&Token::TextToken(ref s)) => self.parse_text(s)?,
            Some(&Token::StartTagToken(ref e)) => self.parse_start_tag(e)?,
            Some(&Token::SelfClosingTagToken(ref e)) => self.parse_self_closing_tag(e)?,
            Some(&Token::CommentToken(ref s)) => self.parse_comment(s)?,
            Some(&Token::EndTagToken(_)) => {
                panic!("Cannot parse_node cause found EndTagToken without context")
            }
            Some(&Token::Illegal) => panic!("Cannot parse_node cause found IllegalToken"),
            Some(&Token::Eof) => panic!("Cannot parse_node cause found EOF"),
            None => panic!("Cannot parse_node cause cannot find next token"),
        };
        Ok(node)
    }

    fn parse_text(&mut self, s: &str) -> Result<Node> {
        Ok(Node::new(NodeType::Text(s.to_string())))
    }

    fn parse_start_tag(&mut self, element_data: &ElementData) -> Result<Node> {
        let mut node = Node::new(NodeType::Element(element_data.clone()));
        let end = Token::EndTagToken(element_data.clone().tag_name);
        loop {
            match self.tokens.peek() {
                Some(t) if t == &&end => {
                    self.tokens.next();
                    break;
                }
                Some(_) => {
                    let child = self.parse_node()?;
                    node.children.push(child);
                }
                None => {}
            }
        }
        Ok(node)
    }

    fn parse_self_closing_tag(&mut self, element_data: &ElementData) -> Result<Node> {
        Ok(Node::new(NodeType::Element(element_data.clone())))
    }

    fn parse_comment(&mut self, comment: &str) -> Result<Node> {
        Ok(Node::new(NodeType::Comment(comment.to_string())))
    }

    fn next_token_is(&mut self, token: &Token) -> bool {
        match self.tokens.peek() {
            Some(t) => *t == token,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::html::dom::{DOMParser, ElementData, Node, NodeType};
    use crate::html::lexer::token::Attributes;
    use crate::html::lexer::Lexer;
    use anyhow::Result;

    fn from_vec(attributes: Vec<(String, String)>) -> Attributes {
        attributes.iter().cloned().collect()
    }

    #[test]
    fn test_parse() -> Result<()> {
        let input = r#"
<html>
    <meta content="html" />
    <div
        className='table'
        id="names">
        <p>Hello</p>
        <p>World</p>
        <!--TODO: implement table-->
    </div>
</html>
"#;
        let tokens = Lexer::new(input).tokens();
        let mut parser = DOMParser::new(&tokens);
        let dom = parser.parse()?;
        dbg!(&dom);
        let expect = Node {
            node_type: NodeType::Element(ElementData::new("html".to_string(), Attributes::new())),
            children: vec![
                Node {
                    node_type: NodeType::Element(ElementData::new(
                        "meta".to_string(),
                        from_vec(vec![("content".to_string(), "html".to_string())]),
                    )),
                    children: vec![],
                },
                Node {
                    node_type: NodeType::Element(ElementData::new(
                        "div".to_string(),
                        from_vec(vec![
                            ("className".to_string(), "table".to_string()),
                            ("id".to_string(), "names".to_string()),
                        ]),
                    )),
                    children: vec![
                        Node {
                            node_type: NodeType::Element(ElementData::new(
                                "p".to_string(),
                                Attributes::new(),
                            )),
                            children: vec![Node::new(NodeType::Text("Hello".to_string()))],
                        },
                        Node {
                            node_type: NodeType::Element(ElementData::new(
                                "p".to_string(),
                                Attributes::new(),
                            )),
                            children: vec![Node::new(NodeType::Text("World".to_string()))],
                        },
                        Node::new(NodeType::Comment("TODO: implement table".to_string())),
                    ],
                },
            ],
        };
        assert_eq!(dom, vec![expect]);
        Ok(())
    }
}
