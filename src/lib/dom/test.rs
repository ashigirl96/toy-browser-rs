// #[cfg(test)]
// mod tests {
//     use std::iter::FromIterator;
//     use crate::DocumentObjectParser;
//     use crate::lib::ElementTagName::*;
//     use crate::lib::Node::*;
//     use crate::lib::NodeKey::*;
//
//     fn new(input: &'static str) -> DocumentObjectParser<'static> {
//         DocumentObjectParser::new(input)
//     }
//
//     #[test]
//     fn test_parse_node() {
//         use crate::lib::*;
//         use crate::lib::ElementAttributes;
//         let input = r#"
// <div class = "name"   id =  "note"  >
//     Hello
//     <div id="space" />
//     <p onClick="console.log(0)">World</p>
//     <!-- TODO: Implement Here -->
// </div>"#;
//         assert_eq!(
//             new(input).parse_node(),
//             Node::Element(Element {
//                 tag_name: Div,
//                 attributes: ElementAttributes::from_iter([
//                     (Id, "note".to_string()),
//                     (Class, "name".to_string())
//                 ]),
//                 children: vec![
//                     Text("Hello".to_string(),),
//                     Node::Element(Element {
//                         tag_name: Div,
//                         attributes: ElementAttributes::from_iter([(Id, "space".to_string())]),
//                         children: vec![]
//                     }),
//                     Node::Element(Element {
//                         tag_name: P,
//                         attributes: ElementAttributes::from_iter([(
//                             Other("onClick".to_string()),
//                             "console.log(0)".to_string()
//                         )]),
//                         children: vec![Text("World".to_string(),),],
//                     },),
//                     Comment("TODO: Implement Here ".to_string(),),
//                 ],
//             },),
//         );
//     }
//
//     #[test]
//     fn test_parse_tag() {
//         let tests = vec![("div ", Div), ("html ", Html), ("h1 ", H1)];
//         for (input, expect) in tests {
//             assert_eq!(new(input).parse_element_tag(), expect);
//         }
//     }
//
//     #[test]
//     fn test_consume_text() {
//         let input = r#"
// Merry Christmas
// Mr. Lawrence.
// "#;
//         assert_eq!(
//             new(input).consume_text(),
//             String::from("Merry Christmas Mr. Lawrence.")
//         );
//     }
//
//     #[test]
//     fn test_consume_for() {
//         let tests = vec![("ab-cde", "ab", '-'), ("abcde", "abcd", 'e')];
//         for (input, expect, next) in tests {
//             let mut parser = new(input);
//             assert_eq!(
//                 parser.consume_for(&|ch| ch.is_alphanumeric(), 4),
//                 String::from(expect),
//             );
//             parser.skip_next_ch(&next);
//         }
//     }
//
//     #[test]
//     fn test() {
//         let input = r#"abcdef"#;
//         let mut p = input.chars().peekable();
//         let mut s = p.by_ref().take(2);
//         dbg!(&s.next());
//         dbg!(&s.next());
//         dbg!(&s.next());
//         dbg!(&p.next());
//         dbg!(&p.next());
//         dbg!(&p.next());
//     }
//
//     #[test]
//     fn test_node_key_from() {
//         use crate::lib::NodeKey;
//         let tests = vec![("id", Id), ("class", Class), ("href", Href)];
//         for (input, expected) in tests {
//             assert_eq!(NodeKey::from(input), expected);
//         }
//     }
//
//     #[test]
//     fn test_element_tag_name_from() {
//         use crate::lib::ElementTagName;
//         let tests = vec![
//             ("html", Html),
//             ("main", Main),
//             ("head", Head),
//             ("title", Title),
//             ("script", Script),
//             ("body", Body),
//             ("div", Div),
//             ("p", P),
//             ("h1", H1),
//             ("h2", H2),
//             ("h3", H3),
//             ("other", Other("other".to_string())),
//         ];
//         for (input, expected) in tests {
//             assert_eq!(ElementTagName::from(input), expected);
//         }
//     }
// }
