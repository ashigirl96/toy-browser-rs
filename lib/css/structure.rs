use std::fmt;
use std::fmt::Debug;

#[derive(Default, PartialEq)]
pub struct StyleSheet {
    pub rules: Vec<Rule>,
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

    #[test]
    fn test_debug_selector() {
        let tests = vec![
            (Selector::Tag("div".to_string()), "div".to_string()),
            (Selector::Class(None, "box".to_string()), ".box".to_string()),
            (
                Selector::Class(
                    Some(Box::new(Selector::Tag("div".to_string()))),
                    "box".to_string(),
                ),
                "div.box".to_string(),
            ),
            (Selector::Id(None, "box".to_string()), "#box".to_string()),
            (
                Selector::Id(
                    Some(Box::new(Selector::Tag("div".to_string()))),
                    "box".to_string(),
                ),
                "div#box".to_string(),
            ),
            (
                Selector::Child(
                    Box::new(Selector::Tag("article".to_string())),
                    Box::new(Selector::Tag("p".to_string())),
                ),
                "article > p".to_string(),
            ),
            (
                Selector::Adjacent(
                    Box::new(Selector::Tag("h1".to_string())),
                    Box::new(Selector::Tag("p".to_string())),
                ),
                "h1 + p".to_string(),
            ),
        ];
        for (actual, expect) in tests {
            assert_eq!(format!("{:?}", actual), expect)
        }
    }
}
