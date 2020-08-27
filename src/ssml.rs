use style::{StyledNode};
use dom::{NodeType, ElementData};
use css::{Value};

fn parse_spec_val(val: &Value) -> String {
    return match val {
        Value::Keyword(s) => s.to_string(),
        Value::ColorValue(c) => format!("#{:02X}{:02X}{:02X}", c.r, c.g, c.b),
        Value::Length(l, _) => format!("{}px", l) 
    }
}

pub fn render_ssml(styled_node: StyledNode, level: usize) -> String {
    let spec_vals = styled_node.specified_values;
    let indent = " ".repeat(level * 2);
    let mut result = String::from("");
    match &styled_node.node.node_type {
        NodeType::Text(s) => {
            result.push_str(indent.as_str());
            result.push_str(s.trim());
            result.push_str("\n");
            return result;
        },
        NodeType::Element(ElementData {tag_name: tag, attributes: _}) => {
            // open tag
            result.push_str(indent.as_str());
            result.push_str("<");
            result.push_str(tag);
            //TODO: move above in order to be able to wrap into tags 
            for (spec_key, spec_val) in spec_vals {
                result.push_str(" ");
                result.push_str(spec_key.as_str());
                result.push_str("=\"");
                result.push_str(parse_spec_val(&spec_val).as_str());
                result.push_str("\"");
            }
            result.push_str(">\n");
            // children
            for child in styled_node.children {
                result.push_str(render_ssml(child, level + 1).as_str());
            }
            // close tag
            result.push_str(indent.as_str());
            result.push_str("</");
            result.push_str(tag);
            result.push_str(">\n");
            return result;
        }, 
    };
}