use crate::css::structure::{PropertyMap, StyleSheet, Value};
use crate::html::dom::{Node, NodeType};
use std::fmt;

pub struct RenderNode<'a> {
    pub node: &'a Node,
    pub styles: PropertyMap<'a>,
    pub children: Vec<RenderNode<'a>>,
}

#[derive(Debug)]
pub enum Display {
    Block,
    Inline,
    InlineBlock,
    Flex,
    None,
}

impl<'a> RenderNode<'a> {
    pub fn new(node: &'a Node, stylesheet: &'a StyleSheet) -> Self {
        let mut children = Vec::new();

        for child in &node.children {
            if let NodeType::Element(_) = child.node_type {
                children.push(Self::new(&child, stylesheet))
            }
        }

        let styles = match node.node_type {
            NodeType::Element(ref e) => stylesheet.get_styles(e),
            _ => PropertyMap::new(),
        };

        Self {
            node,
            styles,
            children,
        }
    }

    pub fn value(&self, name: &str) -> Option<&&'a Value> {
        self.styles.get(name)
    }

    pub fn get_display(&self) -> Display {
        if let Some(s) = self.value("display") {
            return match s {
                Value::Other(ref v) => match v.as_ref() {
                    "block" => Display::Block,
                    "none" => Display::None,
                    "inline-block" => Display::InlineBlock,
                    "flex" => Display::Flex,
                    _ => Display::Inline,
                },
                _ => Display::Inline,
            };
        }
        Display::Inline
    }

    pub fn num_or(&self, name: &str, default: f32) -> f32 {
        if let Some(v) = self.value(name) {
            return match **v {
                Value::Length(n, _) => n,
                _ => default,
            };
        }
        default
    }
}

impl<'a> fmt::Debug for RenderNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?}", self.node, self.styles)
    }
}

#[cfg(test)]
mod tests {
    use crate::css::parser::StyleSheetParser;
    use crate::html::dom::DOMParser;
    use crate::html::lexer::Lexer;
    use crate::render::RenderNode;

    #[test]
    fn test_new() {
        let html = r#"
<div class='table' id="names" />
"#;
        let css = r#"
div#names {
    display: flex;
    margin: 8px;
}
"#;
        let doms = DOMParser::new(&Lexer::new(html).tokens()).parse().unwrap();
        let css = StyleSheetParser::new(&css).parse();
        let render = RenderNode::new(&doms[0], &css);
        dbg!(render.get_display());
        dbg!(render.num_or("margin", 0_f32));
        dbg!(render.node);
    }
}

// div > .table {
// margin: auto ;
// padding : 10.5 px;
// color: #aa11ff22;
// }
//
//
// #answer, h1 {
// display: none;
// }
