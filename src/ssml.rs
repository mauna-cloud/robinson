use css::Value;
use dom::{AttrMap, ElementData, NodeType};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use style::StyledNode;

struct AttrWrapper<'a> {
    attr_map: &'a AttrMap,
}

impl Display for AttrWrapper<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut res = String::new();
        for (k, v) in self.attr_map {
            if k == "class" {
                continue;
            }
            res.push_str(format!(" {}=\"{}\"", k, v.replace("\"", "")).as_str());
        }
        return write!(f, "{}", res);
    }
}

struct Tag<'a> {
    tag_name: &'a str,
    attributes: AttrMap,
}

impl<'a> Tag<'a> {
    fn from_attr(attr_name: &str) -> Self {
        let tag_name = match attr_name {
            "pitch" | "volume" => "prosody",
            "voice-family" | "voice-variant" | "voice-gender" => "voice",
            "level" => "emphasis",
            _ => "",
        };
        Tag {
            tag_name: tag_name,
            attributes: AttrMap::new(),
        }
    }

    fn add_attr<T: Display>(&mut self, attr_name: &str, attr_val: &T) {
        self.attributes
            .insert(attr_name.to_string(), attr_val.to_string());
    }

    fn wrap<T: Display>(&self, tag: &T, indent_level: usize) -> String {
        let indent = " ".repeat(indent_level * 2);
        return format!(
            "{}<{}{}>\n{}{}</{}>\n",
            indent,
            self.tag_name,
            AttrWrapper {
                attr_map: &self.attributes
            },
            tag,
            indent,
            self.tag_name
        );
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        return match self {
            Value::Keyword(s) => write!(f, "{}", s),
            Value::ColorValue(c) => write!(f, "#{:02X}{:02X}{:02X}", c.r, c.g, c.b),
            Value::Length(l, _) => write!(f, "{}px", l),
        };
    }
}

fn render_attrs<T: Display>(attrs: &HashMap<String, T>) -> (Vec<Tag>, String) {
    let mut result_str = String::new();
    let mut result_vec: Vec<Tag> = Vec::new();
    let mut tags: HashMap<String, Tag> = HashMap::new();
    for (k, v) in attrs {
        if k == "class" {
            continue;
        }
        let mut tag = Tag::from_attr(k);
        if tag.tag_name != "" {
            match tags.get_mut(tag.tag_name) {
                Option::None => {
                    tag.add_attr(k, v);
                    tags.insert(tag.tag_name.to_string(), tag);
                    ();
                }
                Option::Some(t) => t.add_attr(k, v),
            };
        } else {
            let attr_str = format!(" {}=\"{}\"", k, v);
            result_str.push_str(attr_str.as_str());
        }
    }
    for (_, v) in tags {
        result_vec.push(v);
    }
    return (result_vec, result_str);
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
            let (_, unspec_attrs) = render_attrs(attrs);
            let (tags, spec_attrs) = render_attrs(spec_vals);
            for tag in tags {
                children_str = tag.wrap(&children_str, level + 1);
            }
            return format!(
                "{}<{}{}{}>\n{}{}</{}>\n",
                indent.as_str(),
                tag,
                unspec_attrs,
                spec_attrs,
                children_str,
                indent.as_str(),
                tag
            );
        }
    };
}
