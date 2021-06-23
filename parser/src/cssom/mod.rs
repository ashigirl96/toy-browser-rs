use crate::prelude::*;

pub mod prelude;
mod test;

impl<'a> StyleSheetParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }

    /// Parse raw CSS input to CSSOM
    ///
    /// ```
    /// use crate::parser::StyleSheetParser;
    /// let css = r#"
    /// div.note, p#name {
    ///     margin: left;
    ///     padding: 10px;
    ///     color: #00FF33;
    ///     display: block;
    /// }
    /// "#;
    /// let mut style_sheet = StyleSheetParser::new(css).parse();
    /// ```
    pub fn parse(&mut self) -> StyleSheet {
        let mut rules = vec![];
        loop {
            if self.input.peek().is_none() {
                break;
            }
            rules.push(self.parse_rule());
            self.skip_whitespace();
        }
        StyleSheet::new(rules)
    }

    /// Parse one CSS Rule, this used in `parse`
    fn parse_rule(&mut self) -> Rule {
        let mut selectors = vec![];
        loop {
            match self.input.peek().unwrap() {
                '{' => {
                    self.bump();
                    break;
                }
                _ => selectors.push(self.parse_selector()),
            }
        }
        let mut declarations = vec![];
        loop {
            match self.input.peek().unwrap() {
                '}' => {
                    self.bump();
                    break;
                }
                _ => declarations.push(self.parse_declaration()),
            }
        }
        Rule::new(selectors, declarations)
    }

    /// Parse Selector from css rule, this used in `parse_rule`
    fn parse_selector(&mut self) -> Selector {
        let selector = self.parse_one_selector();
        if let Some(',') = self.input.peek() {
            self.bump()
        };
        self.skip_whitespace();
        selector
    }

    /// Parse one css selector, this used in `parse_selector`
    fn parse_one_selector(&mut self) -> Selector {
        self.skip_whitespace();
        let left = match self.input.peek() {
            Some('a'..='z' | 'A'..='Z' | '0'..='9') => {
                let tag_name = self.consume_identifier();
                Some(Selector::Tag(ElementTagName::from(tag_name.as_ref())))
            }
            _ => None,
        };
        self.parse_class_selector(left)
    }

    /// Parse tag from css selector, this used in `parse_selector_tag`
    ///
    /// e.g.
    ///   .box  → Selector::Class(None, "box".to_string()))
    ///   p#box → Selector::Id(Some(box (Selector::Tag(P))), "box".to_string()),
    fn parse_class_selector(&mut self, left: Option<Selector>) -> Selector {
        match self.input.peek() {
            Some('.') => {
                self.input.next();
                let class = self.consume_identifier();
                let left = match left {
                    Some(selector) => Selector::Class(Some(box (selector)), class),
                    None => Selector::Class(None, class),
                };
                self.parse_sibling_selector(Some(left))
            }
            Some('#') => {
                self.input.next();
                let id = self.consume_identifier();
                let left = match left {
                    Some(selector) => Selector::Id(Some(box (selector)), id),
                    None => Selector::Id(None, id),
                };
                self.parse_sibling_selector(Some(left))
            }
            Some('+' | '>') => self.parse_sibling_selector(left),
            _ => left.unwrap(),
        }
    }

    /// parse_class_selector内に入れることができるが、可読性のため別けた
    ///
    /// e.g.
    ///   head > div > p
    ///   Selector::Child(
    ///   box (Selector::Tag(Head)),
    ///   box (Selector::Child(box (Selector::Tag(Div)), box (Selector::Tag(P)))),
    fn parse_sibling_selector(&mut self, left: Option<Selector>) -> Selector {
        match self.input.peek() {
            Some('>') => {
                self.input.next();
                let right = self.parse_one_selector();
                match left {
                    Some(selector) => {
                        let left = Selector::Child(box (selector), box (right));
                        self.parse_sibling_selector(Some(left))
                    }
                    None => panic!("Cannot parse right selector"),
                }
            }
            Some('+') => {
                self.input.next();
                let right = self.parse_one_selector();
                match left {
                    Some(selector) => {
                        let left = Selector::Adjacent(box (selector), box (right));
                        self.parse_sibling_selector(Some(left))
                    }
                    None => panic!("Cannot parse right selector"),
                }
            }
            _ => left.unwrap(),
        }
    }

    /// Parse Declaration from css rule, this used in `parse_rule`
    ///
    /// e.g.
    ///   margin: auto; → Declaration::new(Margin, Value::Other("auto".to_string()))
    ///   padding: 10.5px; →  Declaration::new(Padding, Value::Length(10.5, Unit::Px))
    fn parse_declaration(&mut self) -> Declaration {
        let property_name = self.consume_identifier();
        self.skip_next_ch(&':');
        self.skip_whitespace();
        let value = match self.input.peek() {
            Some('#') => {
                self.input.next();
                let r = self.consume_hex(2);
                let g = self.consume_hex(2);
                let b = self.consume_hex(2);
                let a = match self.input.peek() {
                    Some(ch) if matches!(ch, '0'..='9' | 'a'..='z' | 'A'..='Z') => {
                        self.consume_hex(2)
                    }
                    _ => 0,
                };
                Value::Color(Color::new(r, g, b, a))
            }
            Some('0'..='9') => {
                let length = self.consume_number();
                let unit_ident = self.consume_identifier();
                let unit = match unit_ident.as_str() {
                    "px" => Unit::Px,
                    "em" => Unit::Em,
                    _ => Unit::Px,
                };
                Value::Length(length, unit)
            }
            Some('a'..='z' | 'A'..='Z' | '-') => {
                let ident = self.consume_identifier();
                Value::Other(ident)
            }
            _ => panic!("Cannot parse declaration"),
        };
        self.skip_next_ch(&';');
        self.skip_whitespace();
        Declaration::new(DeclarationProperty::by_name(&property_name), value)
    }

    fn consume_identifier(&mut self) -> String {
        self.consume(&|ch| matches!(ch, '0'..='9' | 'a'..='z' | 'A'..='Z' | '_' | '-'))
    }

    fn consume_number(&mut self) -> f32 {
        self.consume(&|ch| matches!(ch, '0'..='9' | '.'))
            .parse()
            .unwrap()
    }

    fn consume_hex(&mut self, n: usize) -> usize {
        usize::from_str_radix(
            &self.consume_for(&|ch| matches!(ch, '0'..='9' | 'a'..='f' | 'A'..='F'), n),
            16,
        )
        .unwrap_or_default()
    }

    /// Get until n-th character strings according to consume_condition
    fn consume_for<F>(&mut self, consume_condition: &F, nth: usize) -> String
    where
        F: Fn(&char) -> bool,
    {
        self.skip_whitespace();
        let mut s = String::new();
        for _ in 0..nth {
            match self.input.next_if(consume_condition) {
                Some(ch) => s.push_str(&ch.to_string()),
                _ => break,
            }
        }
        self.skip_whitespace();
        s
    }

    /// Get strings according to consume_condition
    fn consume<F>(&mut self, consume_condition: &F) -> String
    where
        F: Fn(&char) -> bool,
    {
        self.skip_whitespace();
        let mut s = String::new();
        while let Some(ch) = self.input.next_if(consume_condition) {
            s.push(ch);
        }
        self.skip_whitespace();
        s
    }

    /// Skip specific next character
    fn skip_next_ch(&mut self, ch: &char) {
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
}

impl StyleSheet {
    pub fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }

    /// TODO: ??????
    pub fn get_styles(&self, element: &Element) -> PropertyMap {
        let mut styles = PropertyMap::new();

        for rule in self.rules.iter() {
            for selector in rule.selectors.iter() {
                if selector.matches(element) {
                    for declaration in rule.declarations.iter() {
                        styles.insert(&declaration.property, &declaration.value);
                    }
                    break;
                }
            }
        }
        styles
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

impl Selector {
    /// Elementオブジェクト(e.g. <div id="book" />)を渡されたとき、それに該当するCSS Selectorかどうか判断する
    ///
    /// e.g. it returns true when selector is div#book and element is <div id="book">.
    ///
    /// ```
    /// use parser::prelude::*;
    /// use std::iter::FromIterator;
    /// assert!(Selector::Id(
    ///     Some(Box::new((Selector::Tag(ElementTagName::Div)))),
    ///     "book".to_string()
    /// )
    ///     .matches(&Element::new(
    ///         ElementTagName::Div,
    ///         ElementAttributes::from_iter(vec![(NodeKey::Id, "book".to_string())]),
    ///         vec![],
    ///     )));
    /// ```
    pub fn matches(&self, element: &Element) -> bool {
        match &self {
            Selector::Tag(tag_name) => tag_name == &element.tag_name,
            Selector::Class(Some(box selector), class_name) => {
                let element_class_name = &element.get_classes().unwrap_or_default();
                selector.matches(element) && class_name == element_class_name
            }
            Selector::Class(None, class_name) => {
                let element_class_name = &element.get_classes().unwrap_or_default();
                class_name == element_class_name
            }
            Selector::Id(Some(box selector), id) => {
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

impl Declaration {
    pub fn new(property: DeclarationProperty, value: Value) -> Self {
        Self { property, value }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Other(String::from(""))
    }
}

impl Color {
    pub fn new(r: usize, g: usize, b: usize, a: usize) -> Self {
        Self { r, g, b, a }
    }
}
