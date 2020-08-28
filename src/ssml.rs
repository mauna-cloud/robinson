use css::Value;
use dom::{ElementData, NodeType};
use std::collections::HashMap;
use style::StyledNode;
use std::fmt::{Display, Formatter, Result};

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        return match self {
            Value::Keyword(s) => write!(f, "{}", s),
            Value::ColorValue(c) => write!(f, "#{:02X}{:02X}{:02X}", c.r, c.g, c.b),
            Value::Length(l, _) => write!(f, "{}px", l),
        };
    }
} 

fn render_attrs<T: Display>(attrs: &HashMap<String, T>) -> String {
    let mut result = String::new();
    for (k, v) in attrs {
        let attr_str = format!(" {}=\"{}\"", k, v);
        result.push_str(attr_str.as_str());
    }
    return result;
}

pub fn render_ssml(styled_node: StyledNode, level: usize) -> String {
    let spec_vals = &styled_node.specified_values;
    let indent = " ".repeat(level * 2);
    match &styled_node.node.node_type {
        NodeType::Text(s) => {
            return format!("{}{}\n", indent.as_str(), s.trim());
        }
        NodeType::Element(ElementData {
            tag_name: tag,
            attributes: attrs,
        }) => {
            let mut children_str = String::from("");
            for child in styled_node.children {
                children_str.push_str(render_ssml(child, level + 1).as_str());
            }
            return format!(
                "{}<{}{}{}>\n{}{}</{}>\n",
                indent.as_str(),
                tag,
                render_attrs(attrs),
                render_attrs(spec_vals),
                children_str,
                indent.as_str(),
                tag
            );
        }
    };
}
