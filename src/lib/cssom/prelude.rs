use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::iter::Peekable;
use std::str::Chars;
use super::ElementTagName;

/// TODO: ???
pub type StyleMap = HashMap<DeclarationProperty, DeclarationValue>;

/// Parser that convert raw CSS input to CSSOM(StyleSheet)
#[derive(Debug)]
pub struct StyleSheetParser<'a> {
    pub(crate) input: Peekable<Chars<'a>>,
}

/// CSSOM. i.e. possess some CSS Rule
#[derive(Default, PartialEq, Clone)]
pub struct StyleSheet {
    pub(crate) rules: Vec<Rule>,
    // TODO: impl better
    pub(crate) media_query: Option<String>,
}

/// CSS Rule.
/// h1, h2, div.note, #answer {
///   margin: auto; color: #cc0000
/// }
#[derive(Default, PartialEq, Clone)]
pub struct Rule {
    // h1, h2, h3, div.note, #answer
    pub(crate) selectors: Vec<Selector>,
    // { margin: auto; color: #cc0000; }
    pub(crate) declarations: Vec<Declaration>,
}

/// CSS Selector
/// e.g.
///   h1, .note, #modal, div > p, h1 + p
#[derive(PartialEq, Clone)]
pub enum Selector {
    // h1, div, etc.
    Tag(ElementTagName),
    // .note, div.note, etc.
    Class(Option<Box<Selector>>, String),
    // #note, div#note, etc.
    Id(Option<Box<Selector>>, String),
    // div > p, main > article, etc.
    Child(Box<Selector>, Box<Selector>),
    // h1 + p
    Adjacent(Box<Selector>, Box<Selector>),
    // a:link, a:visited
    Pseudo(Option<Box<Selector>>, PseudoClass),
    // @media (max-width: 700px)
}

/// CSS Declaration
/// e.g.
///   margin: 10px
///   div: #cc0000
///   display: none
#[derive(Default, PartialEq, Clone)]
pub struct Declaration {
    pub property: DeclarationProperty,
    // margin, padding, display, etc.
    pub value: DeclarationValue, // #cc0000, 10px, etc.
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
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
    Width,
    Height,
    Display,
    Color,
    BackgroundColor,
    BorderRadius,
    TextDecoration,
    BoxShadow,
    FontFamily,
    Other(String),
}

impl<'a> From<&'a str> for DeclarationProperty {
    fn from(property_name: &'a str) -> Self {
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
            "width" => Self::Width,
            "height" => Self::Height,
            "display" => Self::Display,
            "color" => Self::Color,
            "background-color" => Self::BackgroundColor,
            "border-radius" => Self::BorderRadius,
            "text-decoration" => Self::TextDecoration,
            "box-shadow" => Self::BoxShadow,
            "font-family" => Self::FontFamily,
            _ => Self::Other(property_name.to_string()),
        }
    }
}

impl<'a> From<&'a str> for Display {
    fn from(key: &'a str) -> Self {
        match key {
            "none" => Self::None,
            "block" => Self::Block,
            "inline" => Self::Inline,
            "inline-block" => Self::InlineBlock,
            "flex" => Self::Flex,
            _ => Self::Block,
        }
    }
}

impl<'a> From<&'a str> for TextDecoration {
    fn from(key: &'a str) -> Self {
        match key {
            "none" => Self::None,
            "underline" => Self::Underline,
            _ => Self::None,
        }
    }
}

impl Default for DeclarationProperty {
    fn default() -> Self {
        Self::Display
    }
}

/// CSS declaration value
#[derive(PartialEq, Clone)]
pub enum DeclarationValue {
    // #cc0000
    Color(Color),
    Length(Length),
    Display(Display),
    TextDecoration(TextDecoration),
    BoxShadow(BoxShadow),
    Other(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Length {
    Actual(f32, Unit),
    Auto,
}

/// Unit of CSS declaration value
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum Display {
    None,
    Block,
    Inline,
    InlineBlock,
    Flex,
}

/// Color of CSS declaration value
#[derive(Default, PartialEq, Clone)]
pub struct Color {
    pub r: usize,
    pub g: usize,
    pub b: usize,
    pub a: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Ord, PartialOrd)]
pub enum TextDecoration {
    None,
    Underline,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BoxShadow {
    pub offset_x: Length,
    pub offset_y: Length,
    pub blur_radius: Length,
    pub spread_radius: Length,
    pub color: Color,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum PseudoClass {
    Link,
    Visited,
    // TODO: impl others...
    Other(String),
}

impl<'a> From<&'a str> for PseudoClass {
    fn from(pseudo_class: &'a str) -> Self {
        match pseudo_class {
            "link" => Self::Link,
            "visited" => Self::Visited,
            _ => Self::Other(pseudo_class.to_string()),
        }
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
            Selector::Pseudo(tag, pc) => match tag {
                Some(selector) => write!(f, "{:?}:{:?}", selector, pc),
                None => write!(f, "#{:?}", pc),
            },
        }
    }
}

impl fmt::Debug for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.property, self.value)
    }
}

impl fmt::Debug for DeclarationValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            DeclarationValue::Color(ref color) => write!(f, "{:?}", color),
            DeclarationValue::Length(length) => {
                let s = match length {
                    Length::Actual(ref x, ref unit) => format!("{}[{:?}] ", x, unit),
                    Length::Auto => "auto ".to_string(),
                };
                write!(f, "{}", s)
            }
            DeclarationValue::Display(ref v) => write!(f, "{:?}", v),
            DeclarationValue::TextDecoration(ref v) => write!(f, "{:?}", v),
            DeclarationValue::BoxShadow(ref v) => write!(f, "{:?}", v),
            DeclarationValue::Other(ref s) => write!(f, "{:?}", s),
        }
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r: {} g: {} b: {} a: {}", self.r, self.g, self.b, self.a)
    }
}
