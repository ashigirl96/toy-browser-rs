use super::*;

#[derive(Debug, Clone)]
pub struct RenderObject {
    pub node: Node,
    pub styles: StyleMap,
    pub children: Vec<RenderObject>,
}

impl RenderObject {
    pub fn build(node: Node, stylesheet: StyleSheet) -> Option<Self> {
        let mut children = Vec::new();
        let styles: StyleMap;
        match node {
            Node::Element(ref e) => {
                if let ElementTagName::Meta | ElementTagName::Script = e.tag_name {
                    return None;
                }
                styles = stylesheet.get_styles(e);
                if let Some(DeclarationValue::Display(Display::None)) =
                    styles.get(&DeclarationProperty::Display)
                {
                    return None;
                }
                for child in e.clone().children {
                    if let Some(ch) = Self::build(child, stylesheet.clone()) {
                        children.push(ch)
                    }
                }
            }
            _ => {
                styles = StyleMap::new();
            }
        }
        let render_object = Self {
            node,
            styles,
            children,
        };
        Some(render_object)
    }

    #[allow(dead_code)]
    pub fn get_display(&self) -> &Display {
        if let Some(s) = self.value(&DeclarationProperty::Display) {
            return match s {
                DeclarationValue::Display(v) => v,
                _ => &Display::Inline,
            };
        }
        &Display::Block
    }

    #[allow(dead_code)]
    pub fn get_length(&self, margin: &DeclarationProperty) -> f64 {
        if let Some(l) = self.value(margin) {
            return match l {
                DeclarationValue::Length(length) => match length {
                    Length::Actual(l, unit) => match unit {
                        Unit::Px => *l as f64,
                        Unit::Em => *l as f64 * 8.0,
                        _ => *l as f64,
                    },
                    Length::Auto => 0.0,
                },
                _ => 0.0,
            };
        }
        0.0
    }

    #[allow(dead_code)]
    pub fn get_width(&self) -> Option<f64> {
        let width = self.get_length(&DeclarationProperty::Width);
        if width != 0.0 {
            return Some(width);
        }
        None
    }

    #[allow(dead_code)]
    pub fn value(&self, name: &DeclarationProperty) -> Option<&DeclarationValue> {
        self.styles.get(name)
    }
}
