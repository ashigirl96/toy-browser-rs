use super::RenderObject;
use crate::prelude::{
    DeclarationProperty, DeclarationValue, Display, ElementTagName, Length, Node, StyleMap,
    StyleSheet,
};

pub mod prelude;
mod test;

impl<'a> RenderObject<'a> {
    pub fn new<'b: 'a>(node: &'b Node, stylesheet: &'b StyleSheet) -> Self {
        Self::build(node, stylesheet).unwrap()
    }

    pub fn build<'b: 'a>(node: &'b Node, stylesheet: &'b StyleSheet) -> Option<Self> {
        let mut children = Vec::new();
        let styles: StyleMap;
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
                    if let Some(ch) = Self::build(child, stylesheet) {
                        children.push(ch)
                    }
                }
            }
            _ => {
                styles = StyleMap::new();
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

    #[allow(dead_code)]
    fn lookup_length(&self, property: &DeclarationProperty) -> Option<Length> {
        if let Some(v) = self.value(property) {
            if let DeclarationValue::Length(ref v) = v {
                return Some(v.clone());
            }
            return None;
        }
        None
    }
}
