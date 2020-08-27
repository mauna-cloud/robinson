use css::Value;
use dom::{ElementData, NodeType};
use std::collections::HashMap;
use style::StyledNode;

fn render_spec_val(val: &Value) -> String {
    return match val {
        Value::Keyword(s) => s.to_string(),
        Value::ColorValue(c) => format!("#{:02X}{:02X}{:02X}", c.r, c.g, c.b),
        Value::Length(l, _) => format!("{}px", l),
    };
}

fn render_spec_vals(vals: HashMap<String, Value>) -> String {
    let mut result = String::from("");
    for (spec_key, spec_val) in vals {
        let props = format!(
            " {}=\"{}\"",
            spec_key.as_str(),
            render_spec_val(&spec_val).as_str()
        );
        result.push_str(props.as_str());
    }
    return result;
}

pub fn render_ssml(styled_node: StyledNode, level: usize) -> String {
    let spec_vals = styled_node.specified_values;
    let indent = " ".repeat(level * 2);
    match &styled_node.node.node_type {
        NodeType::Text(s) => {
            return format!("{}{}\n", indent.as_str(), s.trim());
        }
        NodeType::Element(ElementData {
            tag_name: tag,
            attributes: _,
        }) => {
            let mut children_str = String::from("");
            for child in styled_node.children {
                children_str.push_str(render_ssml(child, level + 1).as_str());
            }
            return format!(
                "{}<{}{}>\n{}{}</{}>\n",
                indent.as_str(),
                tag,
                render_spec_vals(spec_vals),
                children_str,
                indent.as_str(),
                tag
            );
        }
    };
}
