use std::collections::HashMap;
use crate::document::document::Document;
use crate::document::style::StylePropertyList;

pub struct AttrMap {
    attributes: HashMap<String, String>,
}

impl AttrMap {
    pub fn new() -> AttrMap {
        AttrMap {
            attributes: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }

    pub fn all(&self) -> &HashMap<String, String> {
        &self.attributes
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();

        // Make sure keys are always ordered in the same way
        let keys = self.attributes.keys();
        let mut keys: Vec<&String> = keys.collect();
        keys.sort();

        for key in keys {
            let value = self.attributes.get(key).unwrap();
            result.push_str(&format!("{}=\"{}\" ", key, value));
        }
        result.trim_end().to_string()
    }
}


pub struct ElementData {
    /// Element name (ie: P, DIV, IMG etc)
    pub tag_name: String,
    /// Element attributes (src, href, class etc)
    pub attributes: AttrMap,
    /// Is this element self closing (ie: <img />)
    pub self_closing: bool,
    /// Element styles (color, font-size etc)
    pub styles: StylePropertyList,
}

impl ElementData {
    pub fn new(tag_name: String, attributes: Option<AttrMap>, is_self_closing: bool, styles: Option<StylePropertyList>) -> ElementData {
        ElementData {
            tag_name,
            attributes: attributes.unwrap_or(AttrMap::new()),
            self_closing: is_self_closing,
            styles: styles.unwrap_or(StylePropertyList::new()),
        }
    }

    pub fn get_style(&self, key: &str) -> Option<&String> {
        self.styles.properties.get(key)
    }

    #[allow(unused)]
    pub fn get_attribute(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }

    #[allow(unused)]
    pub fn set_attribute(&mut self, key: &str, value: &str) {
        self.attributes.set(key, value);
    }

    pub fn is_self_closing(&self) -> bool {
        self.self_closing
    }
}

pub enum NodeType {
    Text(String),
    Element(ElementData),
}

pub struct Node {
    pub node_id: usize,
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

impl Node {
    pub fn new_text(doc: &Document, data: String) -> Node {
        Node {
            node_id: doc.next_node_id(),
            children: vec![],
            node_type: NodeType::Text(data),
        }
    }

    pub fn new_element(
        doc: &Document,
        tag_name: String,
        attributes: Option<AttrMap>,
        self_closing: bool,
        style: Option<StylePropertyList>
    ) -> Node {
        Node {
            node_id: doc.next_node_id(),
            children: vec![],
            node_type: NodeType::Element(
                ElementData::new(tag_name, attributes, self_closing, style)
            ),
        }
    }
}