use itertools::Itertools;

use crate::prelude::*;
use std::iter::FromIterator;

pub mod prelude;
mod test;

impl<'a> DocumentObjectParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }

    pub fn parse(&mut self) -> Node {
        self.parse_node()
    }

    fn parse_node(&mut self) -> Node {
        match self.peek() {
            Some('<') => {
                self.bump();
                match self.peek() {
                    Some('!') => Node::Comment(self.parse_comment()),
                    Some('a'..='z' | 'A'..='Z') => Node::Element(self.parse_element()),
                    Some('/') => {
                        self.skip_next_end_tag();
                        Node::EndTag
                    }
                    _ => panic!("Cannot parse node"),
                }
            }
            Some(_) => Node::Text(self.consume_text()),
            None => panic!("Cannot parse node"),
        }
    }

    fn parse_element(&mut self) -> Element {
        let tag_name = self.parse_element_tag();
        let attributes = match self.peek() {
            Some('>') => ElementAttributes::new(),
            Some('a'..='z' | 'A'..='Z') => self.parse_element_attributes(),
            _ => panic!("Cannot parse element"),
        };
        let children = match self.peek() {
            Some('/') => {
                self.skip_next_str("/>");
                vec![]
            }
            Some('>') => {
                self.skip_next_ch(&'>');
                self.parse_children()
            }
            _ => panic!("Cannot parse element"),
        };
        Element::new(tag_name, attributes, children)
    }

    fn parse_element_attributes(&mut self) -> ElementAttributes {
        let mut attributes = vec![];
        loop {
            match self.peek() {
                Some('/' | '>') => break,
                Some(_) => {
                    let attribute_key = NodeKey::from(self.consume_identifier().as_ref());
                    self.skip_next_ch(&'=');
                    let attribute_value = self.consume_string();
                    attributes.push((attribute_key, attribute_value));
                }
                _ => panic!("cannot parse element attributes"),
            }
        }
        ElementAttributes::from_iter(attributes)
    }

    fn parse_children(&mut self) -> Vec<Node> {
        let mut children = vec![];
        loop {
            let node = self.parse_node();
            match node {
                Node::EndTag => return children,
                _ => children.push(node),
            };
        }
    }

    fn parse_element_tag(&mut self) -> ElementTagName {
        let tag_name = self.consume_identifier();
        ElementTagName::from(tag_name.as_ref())
    }

    fn parse_comment(&mut self) -> String {
        self.skip_next_str("!--");
        let comment = self.consume(&|ch| !matches!(ch, '-'));
        self.skip_next_str("-->");
        comment
    }

    fn consume_text(&mut self) -> String {
        self.consume(&|ch| !matches!(ch, '<' | '>'))
            .trim_end()
            .to_owned()
            .replace("\n", " ") // TODO: find better practice
    }

    fn consume_identifier(&mut self) -> String {
        self.consume(&|ch| matches!(ch, '0'..='9' | 'a'..='z' | 'A'..='Z' | '_' | '-'))
    }

    fn consume_string(&mut self) -> String {
        self.skip_next_ch(&'"');
        let s = self.consume(&|ch| !matches!(ch, '"'));
        self.skip_next_ch(&'"');
        s
    }

    #[allow(dead_code)]
    fn consume_number(&mut self) -> f32 {
        self.consume(&|ch| matches!(ch, '0'..='9' | '.'))
            .parse()
            .unwrap()
    }

    #[allow(dead_code)]
    fn consume_hex(&mut self, n: usize) -> usize {
        usize::from_str_radix(
            &self.consume_for(&|ch| matches!(ch, '0'..='9' | 'a'..='f' | 'A'..='F'), n),
            16,
        )
        .unwrap_or_default()
    }

    /// Get until n-th character strings according to consume_condition
    #[allow(dead_code)]
    fn consume_for<F>(&mut self, consume_condition: &F, nth: usize) -> String
    where
        F: Fn(&char) -> bool,
    {
        self.skip_whitespace();
        let s = self
            .input
            .by_ref()
            .peeking_take_while(consume_condition)
            .take(nth)
            .join("");
        self.skip_whitespace();
        s
    }

    /// Get strings according to consume_condition
    fn consume<F>(&mut self, consume_condition: &F) -> String
    where
        F: Fn(&char) -> bool,
    {
        self.skip_whitespace();
        let s = self
            .input
            .by_ref()
            .peeking_take_while(consume_condition)
            .join("");
        // 以下の場合でもよかった。nextがconsume_conditionに従わない場合はNoneが返るし、nextもされない
        // while let Some(ch) = self.input.next_if(consume_condition) { s.push(ch); }
        self.skip_whitespace();
        s
    }

    /// Skip eng tag
    ///   e.g. </div>
    fn skip_next_end_tag(&mut self) {
        self.consume(&|ch| !matches!(ch, '>'));
        self.skip_next_ch(&'>');
    }

    /// Skip specific next str
    fn skip_next_str(&mut self, s: &'static str) {
        self.skip_whitespace();
        for ch in s.chars() {
            match self.input.next() {
                Some(c) if c == ch => {}
                _ => panic!("Cannot found {}", ch),
            };
        }
    }

    /// Skip specific next character
    fn skip_next_ch(&mut self, ch: &char) {
        self.skip_whitespace();
        match self.input.next() {
            Some(ref c) if c == ch => {}
            _ => panic!("Cannot found {}", ch),
        };
    }

    fn skip_whitespace(&mut self) {
        while self.input.next_if(|&x| x.is_whitespace()).is_some() {}
    }

    fn bump(&mut self) {
        match self.input.next() {
            Some(c) => c,
            None => panic!("Cannot bump"),
        };
    }

    fn peek(&mut self) -> Option<&char> {
        self.skip_whitespace();
        self.input.peek()
    }
}

impl Element {
    pub fn new(
        tag_name: ElementTagName,
        attributes: ElementAttributes,
        children: Vec<Node>,
    ) -> Self {
        Self {
            tag_name,
            attributes,
            children,
        }
    }

    pub fn get_id(&self) -> Option<&str> {
        self.get_value_by_name(&NodeKey::Id)
    }

    pub fn get_classes(&self) -> Option<&str> {
        self.get_value_by_name(&NodeKey::Class)
    }

    fn get_value_by_name(&self, node_key: &NodeKey) -> Option<&str> {
        for (key, value) in self.attributes.iter() {
            if key == node_key {
                return Some(value);
            }
        }
        None
    }
}

impl<'a> From<&'a str> for ElementTagName {
    fn from(tag_name: &'a str) -> Self {
        match tag_name {
            "html" => Self::Html,
            "main" => Self::Main,
            "head" => Self::Head,
            "div" => Self::Div,
            "p" => Self::P,
            "h1" => Self::H1,
            "h2" => Self::H2,
            "h3" => Self::H3,
            _ => Self::Other(tag_name.to_string()),
        }
    }
}

impl<'a> From<&'a str> for NodeKey {
    fn from(key: &'a str) -> Self {
        match key {
            "id" => Self::Id,
            "class" => Self::Class,
            _ => Self::Other(key.to_string()),
        }
    }
}
