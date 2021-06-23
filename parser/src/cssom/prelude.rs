use crate::prelude::ElementTagName;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::iter::Peekable;
use std::str::Chars;

/// TODO: ???
pub type PropertyMap<'a> = HashMap<&'a DeclarationProperty, &'a Value>;

/// Parser that convert raw CSS input to CSSOM(StyleSheet)
#[derive(Debug)]
pub struct StyleSheetParser<'a> {
    pub(crate) input: Peekable<Chars<'a>>,
}

/// CSSOM. i.e. possess some CSS Rule
#[derive(Default, PartialEq)]
pub struct StyleSheet {
    pub(crate) rules: Vec<Rule>,
}

/// CSS Rule.
/// h1, h2, div.note, #answer {
///   margin: auto; color: #cc0000
/// }
#[derive(Default, PartialEq)]
pub struct Rule {
    pub(crate) selectors: Vec<Selector>, // h1, h2, h3, div.note, #answer
    pub(crate) declarations: Vec<Declaration>, // { margin: auto; color: #cc0000; }
}

/// CSS Selector
/// e.g.
///   h1, .note, #modal, div > p, h1 + p
#[derive(PartialEq)]
pub enum Selector {
    Tag(ElementTagName),                    // h1, div, etc.
    Class(Option<Box<Selector>>, String),   // .note, div.note, etc.
    Id(Option<Box<Selector>>, String),      // #note, div#note, etc.
    Child(Box<Selector>, Box<Selector>),    // div > p, main > article, etc.
    Adjacent(Box<Selector>, Box<Selector>), // h1 + p
}

/// CSS Declaration
/// e.g.
///   margin: 10px
///   div: #cc0000
///   display: none
#[derive(Default, PartialEq)]
pub struct Declaration {
    pub property: DeclarationProperty, // margin, padding, display, etc.
    pub value: Value,                  // #cc0000, 10px, etc.
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum DeclarationProperty {
    Margin,
    MarginLeft,
    MarginRight,
    MarginTop,
    MarginBottom,
    Padding,
    PaddingLeft,
    PaddingRight,
    PaddingTop,
    PaddingBottom,
    Display,
    Color,
    Other(String),
}

impl DeclarationProperty {
    pub(crate) fn by_name(property_name: &str) -> Self {
        match property_name {
            "margin" => Self::Margin,
            "margin-left" => Self::MarginLeft,
            "margin-right" => Self::MarginRight,
            "margin-top" => Self::MarginTop,
            "margin-bottom" => Self::MarginBottom,
            "padding" => Self::Padding,
            "padding-left" => Self::PaddingLeft,
            "padding-right" => Self::PaddingRight,
            "padding-top" => Self::PaddingTop,
            "padding-bottom" => Self::PaddingBottom,
            "display" => Self::Display,
            "color" => Self::Color,
            _ => Self::Other(property_name.to_string()),
        }
    }
}

impl Default for DeclarationProperty {
    fn default() -> Self {
        Self::Display
    }
}

/// CSS declaration value
#[derive(PartialEq)]
pub enum Value {
    Color(Color),      // #cc0000
    Length(f32, Unit), // 20px
    Other(String),     // auto, none
}

/// Unit of CSS declaration value
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

/// Color of CSS declaration value
#[derive(Default, PartialEq)]
pub struct Color {
    pub r: usize,
    pub g: usize,
    pub b: usize,
    pub a: usize,
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

impl fmt::Debug for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.property, self.value)
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

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r: {} g: {} b: {} a: {}", self.r, self.g, self.b, self.a)
    }
}
