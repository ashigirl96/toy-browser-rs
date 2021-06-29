use super::RenderObject;
use crate::prelude::{
    DeclarationProperty, DeclarationValue, Display, ElementTagName, Node, PropertyMap, StyleSheet,
};

pub mod prelude;
mod test;

impl<'a> RenderObject<'a> {
    pub fn new(node: &'a Node, stylesheet: &'a StyleSheet) -> Option<Self> {
        let mut children = Vec::new();
        let styles: PropertyMap;
        match node {
            Node::Element(e) => {
                if let ElementTagName::Meta | ElementTagName::Script = e.tag_name {
                    return None;
                }
                styles = stylesheet.get_styles(e);
                if let Some(DeclarationValue::Display(Display::None)) =
                    styles.get(&DeclarationProperty::Display)
                {
                    return None;
                }
                for child in e.children.iter() {
                    if let Some(ch) = Self::new(child, stylesheet) {
                        children.push(ch)
                    }
                }
            }
            _ => {
                styles = PropertyMap::new();
            }
        };

        let render_object = Self {
            node,
            styles,
            children,
        };

        Some(render_object)
    }

    pub fn get_display(&self) -> &Display {
        if let Some(s) = self.value(&DeclarationProperty::Display) {
            return match s {
                DeclarationValue::Display(v) => v,
                _ => &Display::Inline,
            };
        }
        &Display::Inline
    }

    pub fn value(&self, name: &DeclarationProperty) -> Option<&&'a DeclarationValue> {
        self.styles.get(name)
    }
}
