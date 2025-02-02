use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::document::node::{Node, NodeType, NodeId};

pub struct Document {
    pub root: Option<Node>,
    pub next_node_id: Rc<RefCell<usize>>,
}

impl Document {

    pub fn get_node_by_id(&self, node_id: NodeId) -> Option<&Node> {
        fn get_node_by_id_helper(node: &Node, node_id: NodeId) -> Option<&Node> {
            if node.node_id == node_id {
                return Some(node);
            }

            for child in &node.children {
                let result = get_node_by_id_helper(child, node_id);
                if result.is_some() {
                    return result;
                }
            }

            None
        }

        match &self.root {
            None => None,
            Some(root) => get_node_by_id_helper(root, node_id),
        }
    }

    pub fn set_root(&mut self, root: Node) {
        self.root = Some(root);
    }

    pub fn new() -> Document {
        Document {
            root: None,
            next_node_id: Rc::new(RefCell::new(1)),
        }
    }

    pub fn next_node_id(&self) -> NodeId {
        let id = self.next_node_id.borrow().clone();

        let mut nid = self.next_node_id.borrow_mut();
        *nid += 1;

        NodeId::from(id)
    }
}

pub enum NodeVisit {
    Enter,      // Callback enters the node
    Exit,       // Callback exists the node
}

impl Document {
    pub fn count_elements(&self) -> usize {
        fn count_elements_node(node: &Node) -> usize {
            let mut count = 1;
            for child in &node.children {
                count += count_elements_node(child);
            }
            count
        }

        match &self.root {
            None => 0,
            Some(root) => count_elements_node(root),
        }
    }

    pub fn walk_depth_first<F>(&self, node: &Node, cb: &mut F)
    where
        F: FnMut(&Node, usize, NodeVisit),
    {
        self.walk_depth_first_helper(node, 0, cb);
    }

    fn walk_depth_first_helper<F>(&self, node: &Node, level: usize, cb: &mut F)
    where
        F: FnMut(&Node, usize, NodeVisit),
    {
        cb(node, level, NodeVisit::Enter);
        for child in &node.children {
            self.walk_depth_first_helper(child, level + 1, cb);
        }
        cb(node, level, NodeVisit::Exit);
    }

    pub fn print_tree(&self, writer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error> {
        if self.root.is_none() {
            return Ok(());
        }

        self.walk_depth_first(
            &self.root.as_ref().unwrap(),
            &mut |node, level, visit_mode| {
                let indent = " ".repeat(level * 4);
                match visit_mode {
                    NodeVisit::Enter => {
                        match &node.node_type {
                            NodeType::Text(text) => writeln!(writer, "{}({}) {}", indent, node.node_id, text).unwrap(),
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
    use crate::create_document;

    #[test]
    fn test_walk_depth_first() {
        let document = create_document();

        let mut s = String::new();
        let _ = document.print_tree(&mut s);

        println!("{}", s);
        let result = r#"(10) <html lang="en"/>
    (9) <body />
        (6) <h1 class="title" data-alpine="x-wrap"/>
            (7) header
        </h1>
        (8) <script async="true" src="script.js" type="text/javascript">
        (4) <p class="paragraph"/>
            (5) paragraph
            (2) <strong />
                (3) strong
            </strong>
            (1) <img alt="image" src="image.jpg">
        </p>
    </body>
</html>
"#;
        assert_eq!(result, s);
    }
}