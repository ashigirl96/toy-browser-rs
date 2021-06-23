#[cfg(test)]
mod tests {
    use crate::prelude::DeclarationProperty::{Display, Margin, Padding};
    use crate::prelude::ElementTagName::{Article, Div, Head, H1, P};
    use crate::prelude::NodeKey::{Class, Id};
    use crate::prelude::*;
    use std::collections::HashMap;
    use std::iter::FromIterator;

    fn generate_element(tag_name: ElementTagName, attrs: Vec<(NodeKey, &'static str)>) -> Element {
        let mut attributes = ElementAttributes::new();
        for (key, value) in attrs {
            attributes.insert(key, value.to_string());
        }
        Element::new(tag_name, attributes, vec![])
    }

    #[test]
    fn test_selector_matches_class_name() {
        let div_selector = Selector::Class(Some(box (Selector::Tag(Div))), "box".to_string()); // div.box

        assert!(Selector::Id(
            Some(box (Selector::Tag(ElementTagName::Div))),
            "book".to_string()
        )
        .matches(&Element::new(
            ElementTagName::Div,
            ElementAttributes::from_iter(vec![(NodeKey::Id, "book".to_string())]),
            vec![],
        )));
        let element = generate_element(Div, vec![(Class, "box")]);
        assert!(div_selector.matches(&element));

        let element = generate_element(Div, vec![(Class, "table")]);
        assert!(!div_selector.matches(&element));

        let mut attributes = ElementAttributes::new();
        attributes.insert(Id, "book".to_string());
        assert!(
            Selector::Id(Some(box (Selector::Tag(Div))), "book".to_string())
                .matches(&Element::new(Div, attributes, vec![]))
        );
    }

    #[test]
    fn test_none_selector_matches_class_name() {
        let none_selector = Selector::Class(None, "box".to_string()); // .box
        let element = generate_element(Div, vec![(Class, "box")]);
        assert!(none_selector.matches(&element));

        let element = generate_element(Div, vec![(Class, "table")]);
        assert!(!none_selector.matches(&element));
    }

    #[test]
    fn test_selector_matches_id() {
        let div_selector = Selector::Id(Some(box (Selector::Tag(Div))), "box".to_string()); // div#box

        let element = generate_element(Div, vec![(Id, "box")]);
        assert!(div_selector.matches(&element));

        let element = generate_element(Div, vec![(Id, "table")]);
        assert!(!div_selector.matches(&element));
    }

    #[test]
    fn test_none_selector_matches_id() {
        let none_selector = Selector::Id(None, "box".to_string()); // #box
        let element = generate_element(Div, vec![(Id, "box")]);
        assert!(none_selector.matches(&element));

        let element = generate_element(Div, vec![(Id, "table")]);
        assert!(!none_selector.matches(&element));
    }

    #[test]
    fn test_debug_selector() {
        let tests = vec![
            (Selector::Tag(Div), "div".to_string()),
            (Selector::Class(None, "box".to_string()), ".box".to_string()),
            (
                Selector::Class(Some(box (Selector::Tag(Div))), "box".to_string()),
                "div.box".to_string(),
            ),
            (Selector::Id(None, "box".to_string()), "#box".to_string()),
            (
                Selector::Id(Some(box (Selector::Tag(Div))), "box".to_string()),
                "div#box".to_string(),
            ),
            (
                Selector::Child(box (Selector::Tag(Article)), box (Selector::Tag(P))),
                "article > p".to_string(),
            ),
            (
                Selector::Adjacent(box (Selector::Tag(H1)), box (Selector::Tag(P))),
                "h1 + p".to_string(),
            ),
        ];
        for (actual, expect) in tests {
            assert_eq!(format!("{:?}", actual), expect)
        }
    }

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
            box (Selector::Tag(Div)),
            box (Selector::Class(None, "table".to_string())),
        )];
        let declarations = vec![
            Declaration::new(Margin, Value::Other("auto".to_string())),
            Declaration::new(Padding, Value::Length(10.5, Unit::Px)),
            Declaration::new(
                DeclarationProperty::Color,
                Value::Color(Color::new(0xaa, 0x11, 0xff, 0x22)),
            ),
        ];
        let rule1 = Rule::new(selectors, declarations);

        let selectors = vec![Selector::Id(None, "answer".to_string()), Selector::Tag(H1)];
        let declarations = vec![Declaration::new(Display, Value::Other("none".to_string()))];
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
            box (Selector::Tag(Div)),
            box (Selector::Class(None, "table".to_string())),
        )];
        let declarations = vec![
            Declaration::new(Margin, Value::Other("auto".to_string())),
            Declaration::new(Padding, Value::Length(10.5, Unit::Px)),
            Declaration::new(
                DeclarationProperty::Color,
                Value::Color(Color::new(0xaa, 0x11, 0xff, 0x22)),
            ),
        ];
        let expect = Rule::new(selectors, declarations);
        assert_eq!(parser.parse_rule(), expect);
    }

    #[test]
    fn test_parse_selector_unit() {
        let tests = vec![
            (new("div"), Selector::Tag(Div)),
            (new(".box"), Selector::Class(None, "box".to_string())),
            (
                new("p.box"),
                Selector::Class(Some(box (Selector::Tag(P))), "box".to_string()),
            ),
            (new("#box"), Selector::Id(None, "box".to_string())),
            (
                new("p#box"),
                Selector::Id(Some(box (Selector::Tag(P))), "box".to_string()),
            ),
            (
                new("head > div"),
                Selector::Child(box (Selector::Tag(Head)), box (Selector::Tag(Div))),
            ),
            (
                new("head > div > p"),
                Selector::Child(
                    box (Selector::Tag(Head)),
                    box (Selector::Child(box (Selector::Tag(Div)), box (Selector::Tag(P)))),
                ),
            ),
            (
                new("head + div"),
                Selector::Adjacent(box (Selector::Tag(Head)), box (Selector::Tag(Div))),
            ),
            (
                new("head + div + p"),
                Selector::Adjacent(
                    box (Selector::Tag(Head)),
                    box (Selector::Adjacent(box (Selector::Tag(Div)), box (Selector::Tag(P)))),
                ),
            ),
            (
                new(".table > p"),
                Selector::Child(
                    box (Selector::Class(None, "table".to_string())),
                    box (Selector::Tag(P)),
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
                    box (Selector::Class(Some(box (Selector::Tag(Div))), "table".to_string())),
                    box (Selector::Id(Some(box (Selector::Tag(P))), "box".to_string())),
                ),
            ),
        ];
        for (mut actual, expect) in tests {
            assert_eq!(actual.parse_one_selector(), expect);
        }
    }

    #[test]
    fn test_parse_declaration() {
        let tests = vec![
            (
                new("margin: auto ;"),
                Declaration::new(Margin, Value::Other("auto".to_string())),
            ),
            (
                new("padding : 10.5 px;"),
                Declaration::new(Padding, Value::Length(10.5, Unit::Px)),
            ),
            (
                new("color: #aa11ff22;"),
                Declaration::new(
                    DeclarationProperty::Color,
                    Value::Color(Color::new(0xaa, 0x11, 0xff, 0x22)),
                ),
            ),
            (
                new("color: #aa11ff ;"),
                Declaration::new(
                    DeclarationProperty::Color,
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
        assert!(f32::eq(&new("10.2").consume_number(), &10.2));
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
