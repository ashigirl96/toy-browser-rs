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
            if self.peek().is_none() {
                break;
            }
            rules.push(self.parse_rule());
        }
        StyleSheet::new(rules)
    }

    /// Parse one CSS Rule, this used in `parse`
    fn parse_rule(&mut self) -> Rule {
        let mut selectors = vec![];
        loop {
            match self.peek().unwrap() {
                '{' => {
                    self.bump();
                    break;
                }
                _ => selectors.push(self.parse_selector()),
            }
        }
        let mut declarations = vec![];
        loop {
            match self.peek().unwrap() {
                '}' => {
                    self.bump();
                    break;
                }
                _ => {
                    use crate::prelude::DeclarationProperty::*;
                    let property = DeclarationProperty::from(self.consume_identifier().as_str());
                    self.skip_next_ch(&':');
                    match property {
                        Margin | Padding => declarations.extend(self.parse_declarations(property)),
                        _ => declarations.push(self.parse_declaration(property)),
                    }
                }
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
        selector
    }

    /// Parse one css selector, this used in `parse_selector`
    fn parse_one_selector(&mut self) -> Selector {
        let left = match self.peek() {
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
        match self.peek() {
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

    fn parse_declarations(&mut self, property: DeclarationProperty) -> Vec<Declaration> {
        use crate::prelude::DeclarationProperty::*;
        match property {
            Margin => self.parse_declaration_margin(),
            Padding => self.parse_declaration_padding(),
            _ => panic!("Cannot parse declarations"),
        }
    }

    /// Parse Declaration from css rule, this used in `parse_rule`
    ///
    /// e.g.
    ///   margin: auto; → Declaration::new(Margin, Value::Other("auto".to_string()))
    ///   padding: 10.5px; →  Declaration::new(Padding, Value::Length(10.5, Unit::Px))
    fn parse_declaration(&mut self, property: DeclarationProperty) -> Declaration {
        use crate::prelude::DeclarationProperty::*;
        let declaration = match property {
            MarginLeft | MarginRight | MarginTop | MarginBottom | PaddingLeft | PaddingRight
            | PaddingTop | PaddingBottom | Width | Height | BorderRadius => {
                self.parse_declaration_length(property)
            }
            Color | BackgroundColor => self.parse_declaration_color(property),
            Display => self.parse_declaration_display(),
            Other(s) => self.parse_declaration_other(s),
            _ => panic!("Cannot parse declaration"),
        };
        self.skip_next_ch(&';');
        declaration
    }

    fn parse_declaration_margin(&mut self) -> Vec<Declaration> {
        use crate::prelude::DeclarationProperty::*;
        let (top, right, bottom, left) = self.parse_declaration_lengths();
        vec![
            Declaration::new(MarginTop, DeclarationValue::Length(top)),
            Declaration::new(MarginRight, DeclarationValue::Length(right)),
            Declaration::new(MarginBottom, DeclarationValue::Length(bottom)),
            Declaration::new(MarginLeft, DeclarationValue::Length(left)),
        ]
    }

    fn parse_declaration_padding(&mut self) -> Vec<Declaration> {
        use crate::prelude::DeclarationProperty::*;
        let (top, right, bottom, left) = self.parse_declaration_lengths();
        vec![
            Declaration::new(PaddingTop, DeclarationValue::Length(top)),
            Declaration::new(PaddingRight, DeclarationValue::Length(right)),
            Declaration::new(PaddingBottom, DeclarationValue::Length(bottom)),
            Declaration::new(PaddingLeft, DeclarationValue::Length(left)),
        ]
    }

    fn parse_declaration_length(&mut self, prop: DeclarationProperty) -> Declaration {
        let length = match self.peek() {
            Some('0'..='9') => self.parse_declaration_actual_length(),
            Some(_) => {
                let _ = self.consume_identifier();
                Length::Auto // TODO: Implement other case
            }
            _ => panic!("Cannot parse declaration lengths"),
        };
        Declaration::new(prop, DeclarationValue::Length(length))
    }

    fn parse_declaration_lengths(&mut self) -> (Length, Length, Length, Length) {
        let mut length = vec![];
        let values = loop {
            match self.peek() {
                Some('0'..='9') => length.push(self.parse_declaration_actual_length()),
                Some(';') => break length,
                Some(_) => {
                    let _ = self.consume_identifier();
                    length.push(Length::Auto) // TODO: Implement other case
                }
                _ => panic!("Cannot parse declaration lengths"),
            }
        };
        self.skip_next_ch(&';');
        let values = values.as_slice();

        let (top, right, bottom, left) = match values {
            [top] => (top.clone(), top.clone(), top.clone(), top.clone()),
            [top, right] => (top.clone(), right.clone(), top.clone(), right.clone()),
            [top, right, bottom] => (top.clone(), right.clone(), bottom.clone(), right.clone()),
            [top, right, bottom, left] => {
                (top.clone(), right.clone(), bottom.clone(), left.clone())
            }
            _ => panic!("Cannot parse declaration margin"),
        };
        (top, right, bottom, left)
    }

    fn parse_declaration_display(&mut self) -> Declaration {
        Declaration::new(
            DeclarationProperty::Display,
            DeclarationValue::Display(Display::from(self.consume_identifier().as_str())),
        )
    }

    fn parse_declaration_actual_length(&mut self) -> Length {
        let length = self.consume_number();
        let unit_ident = self.consume_identifier();
        let unit = match unit_ident.as_str() {
            "px" => Unit::Px,
            "em" => Unit::Em,
            _ => Unit::Px,
        };
        Length::Actual(length, unit)
    }

    fn parse_declaration_color(&mut self, property: DeclarationProperty) -> Declaration {
        self.next();
        let r = self.consume_hex(2);
        let g = self.consume_hex(2);
        let b = self.consume_hex(2);
        let a = match self.input.peek() {
            Some(ch) if matches!(ch, '0'..='9' | 'a'..='z' | 'A'..='Z') => self.consume_hex(2),
            _ => 0,
        };
        Declaration::new(property, DeclarationValue::Color(Color::new(r, g, b, a)))
    }

    fn parse_declaration_other(&mut self, s: String) -> Declaration {
        let ident = self.consume_identifier();
        Declaration::new(
            DeclarationProperty::Other(s),
            DeclarationValue::Other(ident),
        )
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

    fn next(&mut self) {
        self.skip_whitespace();
        self.input.next();
    }

    fn peek(&mut self) -> Option<&char> {
        self.skip_whitespace();
        self.input.peek()
    }
}

impl StyleSheet {
    pub fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }

    /// TODO: ??????
    pub fn get_styles(&self, element: &Element) -> StyleMap {
        let mut styles = StyleMap::new();

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
    pub fn new(property: DeclarationProperty, value: DeclarationValue) -> Self {
        Self { property, value }
    }
}

impl Default for DeclarationValue {
    fn default() -> Self {
        DeclarationValue::Other(String::from(""))
    }
}

impl Color {
    pub fn new(r: usize, g: usize, b: usize, a: usize) -> Self {
        Self { r, g, b, a }
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
