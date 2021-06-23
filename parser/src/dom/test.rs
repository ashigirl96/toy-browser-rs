#[cfg(test)]
mod tests {
    use crate::prelude::{ElementAttributes, ElementTagName, NodeKey};
    use crate::DocumentObjectParser;
    use std::iter::FromIterator;

    fn new(input: &'static str) -> DocumentObjectParser<'static> {
        DocumentObjectParser::new(input)
    }

    #[test]
    fn test_parse_node() {
        let input = r#"
<div class = "name"   id =  "note"  >
    Hello
    <div id="space" />
    <p onClick="console.log(0)">World</p>
    <!-- TODO: Implement Here -->
</div>"#;
        dbg!(&new(input).parse_node());
    }

    #[test]
    fn test_parse_tag() {
        use crate::prelude::ElementTagName::*;
        let tests = vec![("div ", Div), ("html ", Html), ("h1 ", H1)];
        for (input, expect) in tests {
            assert_eq!(new(input).parse_element_tag(), expect);
        }
    }

    #[test]
    fn test_consume_text() {
        let input = r#"
Merry Christmas
Mr. Lawrence.
"#;
        assert_eq!(
            new(input).consume_text(),
            String::from("Merry Christmas Mr. Lawrence.")
        );
    }

    #[test]
    fn test_consume_for() {
        let tests = vec![("ab-cde", "ab", '-'), ("abcde", "abcd", 'e')];
        for (input, expect, next) in tests {
            let mut parser = new(input);
            assert_eq!(
                parser.consume_for(&|ch| ch.is_alphanumeric(), 4),
                String::from(expect),
            );
            parser.skip_next_ch(&next);
        }
    }

    #[test]
    fn test() {
        let input = r#"abcdef"#;
        let mut p = input.chars().peekable();
        let mut s = p.by_ref().take(2);
        dbg!(&s.next());
        dbg!(&s.next());
        dbg!(&s.next());
        dbg!(&p.next());
        dbg!(&p.next());
        dbg!(&p.next());
    }

    #[test]
    fn test_parse_element_attributes() {
        use crate::dom::NodeKey::*;
        let tests = vec![(
            r#"id="name""#,
            ElementAttributes::from_iter([(Id, "name".to_string())]),
        )];
        for (input, expected) in tests {
            assert_eq!(new(input).parse_element_attributes(), expected);
        }
    }

    #[test]
    fn test_node_key_from() {
        use crate::dom::NodeKey::*;
        let tests = vec![
            ("id", Id),
            ("class", Class),
            ("href", Other("href".to_string())),
        ];
        for (input, expected) in tests {
            assert_eq!(NodeKey::from(input), expected);
        }
    }

    #[test]
    fn test_element_tag_name_from() {
        use crate::dom::ElementTagName::*;
        let tests = vec![
            ("html", Html),
            ("main", Main),
            ("head", Head),
            ("div", Div),
            ("p", P),
            ("h1", H1),
            ("h2", H2),
            ("h3", H3),
            ("other", Other("other".to_string())),
        ];
        for (input, expected) in tests {
            assert_eq!(ElementTagName::from(input), expected);
        }
    }
}
