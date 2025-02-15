use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::common::document::node::{Node, NodeType, NodeId, AttrMap};
use crate::common::document::style::StylePropertyList;

#[derive(Clone)]
pub struct Document {
    pub arena: HashMap<NodeId, Node>,
    pub root_id: Option<NodeId>,
    next_node_id: Arc<RwLock<NodeId>>,
}

impl Document {

    pub fn new_element(&mut self, tag_name: &str, attributes: Option<AttrMap>, self_closing: bool, style: Option<StylePropertyList>) -> NodeId {
        let node = Node::new_element(self, tag_name.to_string(), attributes, self_closing, style);
        let node_id = node.node_id.clone();
        self.arena.insert(node_id.clone(), node);
        node_id
    }

    pub fn new_text(&mut self, text: &str, style: Option<StylePropertyList>) -> NodeId {
        let node = Node::new_text(self, text.to_string(), style);
        let node_id = node.node_id.clone();
        self.arena.insert(node_id.clone(), node);
        node_id
    }

    pub fn add_child(&mut self, parent_id: NodeId, child_id: NodeId) {
        let parent = self.arena.get_mut(&parent_id).unwrap();
        parent.children.push(child_id);
    }

    pub fn get_node_by_id(&self, node_id: NodeId) -> Option<&Node> {
        self.arena.get(&node_id)
    }

    pub fn set_root(&mut self, root_id: NodeId) {
        self.root_id = Some(root_id);
    }

    pub fn new() -> Document {
        Document {
            arena: HashMap::new(),
            root_id: None,
            next_node_id: Arc::new(RwLock::new(NodeId::new(1))),
        }
    }

    pub fn next_node_id(&self) -> NodeId {
        let mut nid = self.next_node_id.write().unwrap();
        let id = *nid;
        *nid += 1;
        id
    }
}

pub enum NodeVisit {
    Enter,      // Callback enters the node
    Exit,       // Callback exists the node
}

impl Document {
    pub fn count_elements(&self) -> usize {
        self.arena.len()
    }

    pub fn walk_depth_first<F>(&self, node_id: NodeId, cb: &mut F)
    where
        F: FnMut(NodeId, usize, NodeVisit),
    {
        self.walk_depth_first_helper(node_id, 0, cb);
    }

    fn walk_depth_first_helper<F>(&self, node_id: NodeId, level: usize, cb: &mut F)
    where
        F: FnMut(NodeId, usize, NodeVisit),
    {
        cb(node_id, level, NodeVisit::Enter);
        let node = self.get_node_by_id(node_id).unwrap();
        for child_id in &node.children {
            self.walk_depth_first_helper(*child_id, level + 1, cb);
        }
        cb(node_id, level, NodeVisit::Exit);
    }

    pub fn print_tree(&self, writer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error> {
        if self.root_id.is_none() {
            return Ok(());
        }

        self.walk_depth_first(
            self.root_id.unwrap(),
            &mut |node_id, level, visit_mode| {
                let Some(node) = self.get_node_by_id(node_id) else {
                    return;
                };

                let indent = " ".repeat(level * 4);
                match visit_mode {
                    NodeVisit::Enter => {
                        match &node.node_type {
                            NodeType::Text(text, _) => writeln!(writer, "{}({}) {}", indent, node.node_id, text).unwrap(),
                            NodeType::Element(element) => {
                                if element.is_self_closing() {
                                    writeln!(writer, "{}({}) <{} {}>", indent, node.node_id, element.tag_name, element.attributes.to_string()).unwrap();
                                } else {
                                    writeln!(writer, "{}({}) <{} {}/>", indent, node.node_id, element.tag_name, element.attributes.to_string()).unwrap();
                                }
                            }
                        }
                    }
                    NodeVisit::Exit => {
                        if let NodeType::Element(element) = &node.node_type {
                            if ! element.is_self_closing() {
                                writeln!(writer, "{}</{}>", indent, element.tag_name).unwrap();
                            }
                        }
                    }
                }
            },
        );

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crate::document;

    #[test]
    fn test_walk_depth_first() {
        let document = document::create_document();

        let mut s = String::new();
        let _ = document.print_tree(&mut s);

        println!("{}", s);
        let result = r#"(NodeID(10)) <html lang="en"/>
    (NodeID(9)) <body />
        (NodeID(6)) <h1 class="title" data-alpine="x-wrap"/>
            (NodeID(7)) header
        </h1>
        (NodeID(8)) <script async="true" src="script.js" type="text/javascript">
        (NodeID(4)) <p class="paragraph"/>
            (NodeID(2)) <strong />
                (NodeID(3)) strong
            </strong>
            (NodeID(1)) <img alt="image" src="image.jpg">
        </p>
    </body>
</html>
"#;
        assert_eq!(result, s);
    }
}