use crate::css::structure::{Color, Declaration, Rule, Selector, StyleSheet, Unit, Value};
use std::iter::Peekable;
use std::option::Option::Some;
use std::str::Chars;

#[derive(Debug)]
pub struct StyleSheetParser<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> StyleSheetParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }

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

    fn parse_selector(&mut self) -> Selector {
        let selector = self.parse_selector_unit();
        if let Some(',') = self.input.peek() {
            self.bump()
        };
        self.skip_whitespace();
        selector
    }

    fn parse_selector_unit(&mut self) -> Selector {
        self.skip_whitespace();
        let left = match self.input.peek() {
            Some('a'..='z' | 'A'..='Z' | '0'..='9') => {
                let tag_name = self.consume_identifier();
                Some(Selector::Tag(tag_name))
            }
            _ => None,
        };
        self.parse_class_selector(left)
    }

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

    // parse_class_selector内に入れることができるが、可読性のため別けた
    fn parse_sibling_selector(&mut self, left: Option<Selector>) -> Selector {
        match self.input.peek() {
            Some('>') => {
                self.input.next();
                let right = self.parse_selector_unit();
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
                let right = self.parse_selector_unit();
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

    fn parse_declaration(&mut self) -> Declaration {
        let property = self.consume_identifier();
        self.skip_next_ch(&':');
        self.skip_whitespace();
        let value = match self.input.peek() {
            Some('#') => {
                self.input.next();
                let r = self.consume_hex(2);
                let g = self.consume_hex(2);
                let b = self.consume_hex(2);
                let a = match self.input.peek() {
                    Some(ch) if matches!(ch, '0'..='9' | 'a'..='z') => self.consume_hex(2),
                    _ => 0,
                };
                Value::Color(Color::new(r, g, b, a))
            }
            Some('0'..='9') => {
                let length = self.consume_number();
                let unit_ident = self.consume_identifier();
                let unit = match unit_ident.as_str() {
                    "px" => Unit::Px,
                    _ => Unit::Px,
                };
                Value::Length(length, unit)
            }
            Some('a'..='z' | 'A'..='Z') => {
                let ident = self.consume_identifier();
                Value::Other(ident)
            }
            _ => panic!("Cannot parse declaration"),
        };
        self.skip_next_ch(&';');
        self.skip_whitespace();
        Declaration::new(property, value)
    }

    fn consume_identifier(&mut self) -> String {
        self.consume(&|ch| matches!(ch, '0'..='9' | 'a'..='z' | 'A'..='Z' | '_'))
    }

    fn consume_number(&mut self) -> f32 {
        self.consume(&|ch| matches!(ch, '0'..='9' | '.'))
            .parse()
            .unwrap()
    }

    fn consume_hex(&mut self, n: usize) -> usize {
        usize::from_str_radix(
            &self.consume_for(&|ch| matches!(ch, '0'..='9' | 'a'..='f'), n),
            16,
        )
        .unwrap_or_default()
    }

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

    fn consume<F>(&mut self, consume_condition: &F) -> String
    where
        F: Fn(&char) -> bool,
    {
        self.skip_whitespace();
        let mut s = String::new();
        while let Some(ch) = self.input.next_if(consume_condition) {
            s.push_str(&ch.to_string());
        }
        self.skip_whitespace();
        s
    }

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

#[cfg(test)]
mod tests {
    use crate::css::parser::StyleSheetParser;
    use crate::css::structure::{Color, Declaration, Rule, Selector, StyleSheet, Unit, Value};

    fn new(input: &str) -> StyleSheetParser {
        StyleSheetParser::new(input)
    }

    #[test]
    fn test_parse() {
        let mut parser = StyleSheetParser::new(
            r#"
div > .table {
    margin: auto ; 
    padding : 10.5 px; 
    color: #aa11ff22; 
}


#answer, h1 {
    display: none;
}
"#,
        );
        let selectors = vec![Selector::Child(
            box (Selector::Tag("div".to_string())),
            box (Selector::Class(None, "table".to_string())),
        )];
        let declarations = vec![
            Declaration::new("margin".to_string(), Value::Other("auto".to_string())),
            Declaration::new("padding".to_string(), Value::Length(10.5, Unit::Px)),
            Declaration::new(
                "color".to_string(),
                Value::Color(Color::new(0xaa, 0x11, 0xff, 0x22)),
            ),
        ];
        let rule1 = Rule::new(selectors, declarations);

        let selectors = vec![
            Selector::Id(None, "answer".to_string()),
            Selector::Tag("h1".to_string()),
        ];
        let declarations = vec![Declaration::new(
            "display".to_string(),
            Value::Other("none".to_string()),
        )];
        let rule2 = Rule::new(selectors, declarations);

        let expect = StyleSheet::new(vec![rule1, rule2]);
        assert_eq!(parser.parse(), expect);
    }

    #[test]
    fn test_parse_rule() {
        let mut parser = StyleSheetParser::new(
            r#"
div > .table {
    margin: auto ; 
    padding : 10.5 px; 
    color: #aa11ff22; 
}"#,
        );
        let selectors = vec![Selector::Child(
            box (Selector::Tag("div".to_string())),
            box (Selector::Class(None, "table".to_string())),
        )];
        let declarations = vec![
            Declaration::new("margin".to_string(), Value::Other("auto".to_string())),
            Declaration::new("padding".to_string(), Value::Length(10.5, Unit::Px)),
            Declaration::new(
                "color".to_string(),
                Value::Color(Color::new(0xaa, 0x11, 0xff, 0x22)),
            ),
        ];
        let expect = Rule::new(selectors, declarations);
        assert_eq!(parser.parse_rule(), expect);
    }

    #[test]
    fn test_parse_selector_unit() {
        let tests = vec![
            (new("div"), Selector::Tag("div".to_string())),
            (new(".box"), Selector::Class(None, "box".to_string())),
            (
                new("p.box"),
                Selector::Class(
                    Some(box (Selector::Tag("p".to_string()))),
                    "box".to_string(),
                ),
            ),
            (new("#box"), Selector::Id(None, "box".to_string())),
            (
                new("p#box"),
                Selector::Id(
                    Some(box (Selector::Tag("p".to_string()))),
                    "box".to_string(),
                ),
            ),
            (
                new("head > div"),
                Selector::Child(
                    box (Selector::Tag("head".to_string())),
                    box (Selector::Tag("div".to_string())),
                ),
            ),
            (
                new("head > div > p"),
                Selector::Child(
                    box (Selector::Tag("head".to_string())),
                    box (Selector::Child(
                        box (Selector::Tag("div".to_string())),
                        box (Selector::Tag("p".to_string())),
                    )),
                ),
            ),
            (
                new("head + div"),
                Selector::Adjacent(
                    box (Selector::Tag("head".to_string())),
                    box (Selector::Tag("div".to_string())),
                ),
            ),
            (
                new("head + div + p"),
                Selector::Adjacent(
                    box (Selector::Tag("head".to_string())),
                    box (Selector::Adjacent(
                        box (Selector::Tag("div".to_string())),
                        box (Selector::Tag("p".to_string())),
                    )),
                ),
            ),
            (
                new(".table > p"),
                Selector::Child(
                    box (Selector::Class(None, "table".to_string())),
                    box (Selector::Tag("p".to_string())),
                ),
            ),
            (
                new(".table > #box"),
                Selector::Child(
                    box (Selector::Class(None, "table".to_string())),
                    box (Selector::Id(None, "box".to_string())),
                ),
            ),
            (
                new("div.table > p#box"),
                Selector::Child(
                    box (Selector::Class(
                        Some(box (Selector::Tag("div".to_string()))),
                        "table".to_string(),
                    )),
                    box (Selector::Id(
                        Some(box (Selector::Tag("p".to_string()))),
                        "box".to_string(),
                    )),
                ),
            ),
        ];
        for (mut actual, expect) in tests {
            assert_eq!(actual.parse_selector_unit(), expect);
        }
    }

    #[test]
    fn test_parse_declaration() {
        let tests = vec![
            (
                new("margin: auto ;"),
                Declaration::new("margin".to_string(), Value::Other("auto".to_string())),
            ),
            (
                new("padding : 10.5 px;"),
                Declaration::new("padding".to_string(), Value::Length(10.5, Unit::Px)),
            ),
            (
                new("color: #aa11ff22;"),
                Declaration::new(
                    "color".to_string(),
                    Value::Color(Color::new(0xaa, 0x11, 0xff, 0x22)),
                ),
            ),
            (
                new("color: #aa11ff ;"),
                Declaration::new(
                    "color".to_string(),
                    Value::Color(Color::new(0xaa, 0x11, 0xff, 0x00)),
                ),
            ),
        ];
        for (mut parser, expect) in tests {
            assert_eq!(parser.parse_declaration(), expect)
        }
    }

    #[test]
    fn test_consume_identifier() {
        assert_eq!(new("h_1, h_2").consume_identifier(), "h_1".to_string());
    }

    #[test]
    fn test_consume_hex() {
        assert_eq!(new("ffaa").consume_hex(2), 0xff);
    }

    #[test]
    fn test_consume_number() {
        assert_eq!(new("10.2").consume_number(), 10.2 as f32);
    }

    #[test]
    fn test_consume_for() {
        assert_eq!(
            new("h1h2").consume_for(&|ch| ch.is_ascii_alphanumeric(), 2),
            "h1".to_string(),
        );
    }

    #[test]
    fn test_consume() {
        assert_eq!(
            new("h1, h2").consume(&|ch| ch.is_ascii_alphanumeric()),
            "h1".to_string(),
        );
    }

    #[test]
    fn test_skip_whitespace() {
        let mut parser = new("  x ");
        parser.skip_whitespace();
        assert_eq!(parser.input.next().unwrap(), 'x');
    }
}
