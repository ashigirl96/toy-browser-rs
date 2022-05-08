use std::collections::BTreeMap;
use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

/// Parser that convert raw HTML input to DOM
pub struct DocumentObjectParser<'a> {
    pub(crate) input: Peekable<Chars<'a>>,
}

/// HTML node
/// e.g.
///   <div class="test" />
///   Hello, world
///   <!-- implement here -->
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Text(String),
    Style(String),
    Element(Element),
    Comment(String),
    EndTag, // Document,
}

impl Node {
    // TODO: refactor
    pub fn extract_style(&self) -> String {
        if let Node::Style(style) = self.find_style().unwrap().children[0].clone() {
            style
        } else {
            "".to_string()
        }
    }
    fn find_style(&self) -> Option<Element> {
        match self {
            Node::Element(elem) => elem.find_style(),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn name(&self) -> String {
        match self {
            Node::Element(ref elem) => elem.tag_name.to_string(),
            _ => "".to_string(),
        }
    }
}

/// HTML Element
/// e.g.
///   <div class="table" id="consultation">
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Element {
    pub tag_name: ElementTagName,
    pub attributes: ElementAttributes,
    pub children: Vec<Node>,
}

impl Element {
    fn find_style(&self) -> Option<Self> {
        if self.children.is_empty() {
            return None;
        }
        if self.is_style() {
            return Some(self.clone());
        }
        for child in self.children.iter() {
            let elem = child.find_style();
            if elem.is_some() {
                return elem;
            }
        }
        None
    }

    fn is_style(&self) -> bool {
        self.tag_name == ElementTagName::Style
    }
}

/// HTML Element tagName
/// e.g. div of <div>
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ElementTagName {
    Html,
    Main,
    Div,
    Head,
    Meta,
    Body,
    Title,
    Script,
    Style,
    #[allow(dead_code)]
    Article,
    P,
    H1,
    H2,
    H3,
    A,
    Other(String),
}

/// HTML Element attributes
/// e.g.
///   class: "table"
///   id: "consultation"
pub type ElementAttributes = BTreeMap<NodeKey, String>;

/// HTML Element key
/// e.g. id of <div id="test">
#[derive(Debug, PartialEq, Eq, Clone, Hash, Ord, PartialOrd)]
pub enum NodeKey {
    Id,
    Class,
    Href,
    Other(String),
}

impl fmt::Display for ElementTagName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self).to_lowercase();
        write!(f, "{}", s)
    }
}

impl<'a> From<&'a str> for ElementTagName {
    fn from(tag_name: &'a str) -> Self {
        match tag_name {
            "html" => Self::Html,
            "main" => Self::Main,
            "head" => Self::Head,
            "meta" => Self::Meta,
            "title" => Self::Title,
            "body" => Self::Body,
            "style" => Self::Style,
            "script" => Self::Script,
            "div" => Self::Div,
            "p" => Self::P,
            "h1" => Self::H1,
            "h2" => Self::H2,
            "h3" => Self::H3,
            "a" => Self::A,
            _ => Self::Other(tag_name.to_string()),
        }
    }
}

impl<'a> From<&'a str> for NodeKey {
    fn from(key: &'a str) -> Self {
        match key {
            "id" => Self::Id,
            "class" => Self::Class,
            "href" => Self::Href,
            _ => Self::Other(key.to_string()),
        }
    }
}
