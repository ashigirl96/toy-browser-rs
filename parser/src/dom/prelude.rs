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
    Element(Element),
    Comment(String),
    EndTag, // Document,
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

/// HTML Element tagName
/// e.g. div of <div>
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ElementTagName {
    Html,
    Main,
    Div,
    Head,
    Body,
    Title,
    Script,
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
