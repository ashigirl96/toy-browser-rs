use crate::prelude::{Node, StyleMap};
use std::fmt;
use std::fmt::Formatter;

pub struct RenderObject<'a> {
    pub node: &'a Node,
    pub styles: StyleMap<'a>,
    pub children: Vec<RenderObject<'a>>,
}

impl<'a> fmt::Debug for RenderObject<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n{:?}", pretty_print(self, 0))
    }
}

impl<'a> fmt::Display for RenderObject<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\n{}", pretty_print(self, 0))
    }
}

fn pretty_print(render_object: &RenderObject, indent_size: usize) -> String {
    let indent = " ".repeat(indent_size);
    let parent = match render_object.node {
        Node::Text(v) => {
            format!("{}{}", indent, v)
        }
        Node::Element(e) => {
            if render_object.styles.is_empty() {
                format!("{}<{}>", indent, e.tag_name.to_string())
            } else {
                format!(
                    r#"{}<{} styles={:?}>"#,
                    indent,
                    e.tag_name.to_string(),
                    render_object.styles
                )
            }
        }
        _ => String::from(""),
    };
    let mut children = String::new();
    for child in render_object.children.iter() {
        children.push_str(pretty_print(child, indent_size + 2).as_str())
    }

    let close = match render_object.node {
        Node::Element(e) => {
            format!("{}</{}>\n", indent, e.tag_name.to_string())
        }
        _ => String::new(),
    };

    format!("{}\n{}{}", parent, children, close)
}
