use crate::html::lexer::token::ElementData;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;

pub type PropertyMap<'a> = HashMap<&'a str, &'a Value>;

#[derive(Default, PartialEq)]
pub struct StyleSheet {
    pub rules: Vec<Rule>,
}

impl StyleSheet {
    pub fn get_styles(&self, element: &ElementData) -> PropertyMap {
        let mut styles = PropertyMap::new();

        for rule in &self.rules {
            for selector in &rule.selectors {
                if selector.matches(element) {
                    for declaration in &rule.declarations {
                        styles.insert(&declaration.property, &declaration.value);
                    }
                    break;
                }
            }
        }
        styles
    }
}

// h1, h2, div.note, #answer { margin: auto; color: #cc0000 }
#[derive(Default, PartialEq)]
pub struct Rule {
    pub selectors: Vec<Selector>,       // h1, h2, h3, div.note, #answer
    pub declarations: Vec<Declaration>, // { margin: auto; color: #cc0000; }
}

// div > p
// div + p
// h1
// answer
// .note
#[derive(PartialEq)]
pub enum Selector {
    Tag(String),                            // h1, div, etc.
    Class(Option<Box<Selector>>, String),   // .note, div.note, etc.
    Id(Option<Box<Selector>>, String),      // #note, div#note, etc.
    Child(Box<Selector>, Box<Selector>),    // article > p
    Adjacent(Box<Selector>, Box<Selector>), // h1 + p
}

impl Selector {
    pub fn matches(&self, element: &ElementData) -> bool {
        match &self {
            Selector::Tag(tag_name) => tag_name == &element.tag_name,
            Selector::Class(Option::Some(box selector), class_name) => {
                let element_class_name = &element.get_classes().unwrap_or_default();
                selector.matches(element) && class_name == element_class_name
            }
            Selector::Class(None, class_name) => {
                let element_class_name = &element.get_classes().unwrap_or_default();
                class_name == element_class_name
            }
            Selector::Id(Option::Some(box selector), id) => {
                let element_id = &element.get_id().unwrap_or_default();
                selector.matches(element) && id == element_id
            }
            Selector::Id(None, id) => {
                let element_id = &element.get_id().unwrap_or_default();
                id == element_id
            }
            _ => false,
        }
    }
}

// margin: 10px
// div: #cc0000
// display: none
#[derive(Default, PartialEq)]
pub struct Declaration {
    pub property: String, // margin, padding, display, etc.
    pub value: Value,     // #cc0000, 10px, etc.
}

#[derive(PartialEq)]
pub enum Value {
    Color(Color),      // #cc0000
    Length(f32, Unit), // 20px
    Other(String),     // auto, none
}

#[derive(Debug, PartialEq)]
pub enum Unit {
    Em,
    Ex,
    Ch,
    Rem,
    Vh,
    Vw,
    Vmin,
    Vmax,
    Px,
    Mm,
    Q,
    Cm,
    In,
    Pt,
    Pc,
    Pct,
}

#[derive(Default, PartialEq)]
pub struct Color {
    pub r: usize,
    pub g: usize,
    pub b: usize,
    pub a: usize,
}

impl StyleSheet {
    pub fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }
}

impl fmt::Debug for StyleSheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rules = String::new();
        for rule in &self.rules {
            if !rules.is_empty() {
                rules.push_str("\n\n");
            }
            rules.push_str(&format!("{:?}", rule));
        }
        write!(f, "{}", rules)
    }
}

impl Rule {
    pub fn new(selectors: Vec<Selector>, declarations: Vec<Declaration>) -> Self {
        Self {
            selectors,
            declarations,
        }
    }
}

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut selectors = String::new();
        let mut declarations = String::new();
        let tab = "\t";

        for selector in &self.selectors {
            if !selectors.is_empty() {
                selectors.push_str(", ");
            }
            selectors.push_str(&format!("{:?}", selector));
        }

        for declaration in &self.declarations {
            declarations.push_str(tab);
            declarations.push_str(&format!("{:?}", declaration));
            declarations.push('\n');
        }

        write!(f, "{} {{\n{}}}", selectors, declarations)
    }
}

impl fmt::Debug for Selector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Selector::Tag(ref tag) => write!(f, "{}", tag),
            Selector::Class(s, class) => match s {
                Some(selector) => write!(f, "{:?}.{}", selector, class),
                None => write!(f, ".{}", class),
            },
            Selector::Id(s, id) => match s {
                Some(selector) => write!(f, "{:?}#{}", selector, id),
                None => write!(f, "#{}", id),
            },
            Selector::Child(p, c) => write!(f, "{:?} > {:?}", p, c),
            Selector::Adjacent(l, r) => write!(f, "{:?} + {:?}", l, r),
        }
    }
}

impl Declaration {
    pub fn new(property: String, value: Value) -> Self {
        Self { property, value }
    }
}

impl fmt::Debug for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.property, self.value)
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Other(String::from(""))
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Value::Color(ref color) => write!(f, "{:?}", color),
            Value::Length(ref x, ref unit) => write!(f, "{}[{:?}]", x, unit),
            Value::Other(ref s) => write!(f, "{:?}", s),
        }
    }
}

impl Color {
    pub fn new(r: usize, g: usize, b: usize, a: usize) -> Self {
        Self { r, g, b, a }
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r: {} g: {} b: {} a: {}", self.r, self.g, self.b, self.a)
    }
}

#[cfg(test)]
mod tests {
    use crate::css::structure::Selector;
    use crate::html::lexer::token::{Attributes, ElementData};

    fn generate_element(
        tag_name: &'static str,
        attrs: Vec<(&'static str, &'static str)>,
    ) -> ElementData {
        let mut attributes = Attributes::new();
        for (key, value) in attrs {
            attributes.insert(key.to_string(), value.to_string());
        }
        ElementData::new(tag_name.to_string(), attributes)
    }

    #[test]
    fn test_selector_matches_class_name() {
        let div_selector = Selector::Class(
            Some(box (Selector::Tag("div".to_string()))),
            "box".to_string(),
        ); // div.box

        let element = generate_element("div", vec![("class", "box")]);
        assert!(div_selector.matches(&element));

        let element = generate_element("div", vec![("class", "table")]);
        assert!(!div_selector.matches(&element));
    }

    #[test]
    fn test_none_selector_matches_class_name() {
        let none_selector = Selector::Class(None, "box".to_string()); // .box
        let element = generate_element("div", vec![("class", "box")]);
        assert!(none_selector.matches(&element));

        let element = generate_element("div", vec![("class", "table")]);
        assert!(!none_selector.matches(&element));
    }

    #[test]
    fn test_selector_matches_id() {
        let div_selector = Selector::Id(
            Some(box (Selector::Tag("div".to_string()))),
            "box".to_string(),
        ); // div#box

        let element = generate_element("div", vec![("id", "box")]);
        assert!(div_selector.matches(&element));

        let element = generate_element("div", vec![("id", "table")]);
        assert!(!div_selector.matches(&element));
    }

    #[test]
    fn test_none_selector_matches_id() {
        let none_selector = Selector::Id(None, "box".to_string()); // #box
        let element = generate_element("div", vec![("id", "box")]);
        assert!(none_selector.matches(&element));

        let element = generate_element("div", vec![("id", "table")]);
        assert!(!none_selector.matches(&element));
    }

    #[test]
    fn test_debug_selector() {
        let tests = vec![
            (Selector::Tag("div".to_string()), "div".to_string()),
            (Selector::Class(None, "box".to_string()), ".box".to_string()),
            (
                Selector::Class(
                    Some(box (Selector::Tag("div".to_string()))),
                    "box".to_string(),
                ),
                "div.box".to_string(),
            ),
            (Selector::Id(None, "box".to_string()), "#box".to_string()),
            (
                Selector::Id(
                    Some(box (Selector::Tag("div".to_string()))),
                    "box".to_string(),
                ),
                "div#box".to_string(),
            ),
            (
                Selector::Child(
                    box (Selector::Tag("article".to_string())),
                    box (Selector::Tag("p".to_string())),
                ),
                "article > p".to_string(),
            ),
            (
                Selector::Adjacent(
                    box (Selector::Tag("h1".to_string())),
                    box (Selector::Tag("p".to_string())),
                ),
                "h1 + p".to_string(),
            ),
        ];
        for (actual, expect) in tests {
            assert_eq!(format!("{:?}", actual), expect)
        }
    }
}
