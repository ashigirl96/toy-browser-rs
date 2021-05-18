use std::fmt;
use std::fmt::Debug;

#[derive(Default)]
pub struct StyleSheet {
    pub rules: Vec<Rule>,
}

// h1, h2, div.note, #answer { margin: auto, color: #cc0000 }
#[derive(Default)]
pub struct Rule {
    pub selectors: Vec<Selector>,       // h1, h2, h3, div.note, #answer
    pub declarations: Vec<Declaration>, // { margin: auto; color: #cc0000; }
}

// div > p
// div + p
#[derive(Default)]
pub struct Selector {
    pub simple: Vec<SimpleSelector>, // div, p
    pub combinators: Vec<char>,      // >, +, etc.
}

// h1
// answer
// .note
#[derive(Default)]
pub struct SimpleSelector {
    pub tag_name: Option<String>, // h1, h2, h3, div
    pub id: Option<String>,       // answer
    pub classes: Vec<String>,     // .note
}

// margin: 10px
// div: #cc0000
// display: none
#[derive(Debug, Default)]
pub struct Declaration {
    pub property: String, // margin, padding, display, etc.
    pub value: Value,     // #cc0000, 10px, etc.
}

pub enum Value {
    Color(Color),      // #cc0000
    Length(f32, Unit), // 20px
    Other(String),     // auto, none
}

#[derive(Debug)]
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

#[derive(Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
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
            if rules.len() > 0 {
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
            declarations.push_str("\n");
        }

        write!(f, "{} {{\n{}}}", selectors, declarations)
    }
}

impl Selector {
    pub fn new(simple: Vec<SimpleSelector>, combinators: Vec<char>) -> Self {
        Self {
            simple,
            combinators,
        }
    }
}

impl fmt::Debug for Selector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut simple = String::new();
        for ss in &self.simple {
            if !simple.is_empty() {
                simple.push_str(", ");
            }
            simple.push_str(&format!("{:?}", ss));
        }
        write!(f, "{}", simple)
    }
}

impl SimpleSelector {
    pub fn new(tag_name: Option<String>, id: Option<String>, classes: Vec<String>) -> Self {
        Self {
            tag_name,
            id,
            classes,
        }
    }
}

impl fmt::Debug for SimpleSelector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        if let Some(ref t) = self.tag_name {
            result.push_str(t)
        }

        if let Some(ref s) = self.id {
            result.push('#');
            result.push_str(s);
        }

        for class in &self.classes {
            result.push('.');
            result.push_str(class);
        }

        write!(f, "{}", result)
    }
}

impl Declaration {
    pub fn new(property: String, value: Value) -> Self {
        Self { property, value }
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
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r: {} g: {} b: {} a: {}", self.r, self.g, self.b, self.a)
    }
}
