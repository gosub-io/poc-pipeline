use std::cell::{Ref, RefCell, RefMut};
use crate::layouter::{LayoutElementId, LayoutTree};

pub type LayerId = usize;

#[derive(Clone)]
pub(crate) struct Layer {
    /// Layer ID
    pub layer_id: LayerId,
    /// Order of the layer
    pub order: isize,
    /// Elements in this layer
    pub elements: Vec<LayoutElementId>
}

impl Layer {
    pub fn new(layer_id: LayerId, order: isize) -> Layer {
        Layer {
            layer_id,
            order,
            elements: Vec::new()
        }
    }

    fn add_element(&mut self, element_id: LayoutElementId) {
        self.elements.push(element_id);
    }
}

impl std::fmt::Debug for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Layer")
            .field("elements", &self.elements)
            .finish()
    }
}

#[derive(Clone)]
pub(crate) struct LayerList {
    pub layout_tree: LayoutTree,
    pub layers: RefCell<Vec<Layer>>,
    next_layer_id: RefCell<LayerId>,
}

impl std::fmt::Debug for LayerList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LayerList")
            .field("layout_tree", &self.layout_tree)
            .field("layers", &self.layers)
            .finish()
    }
}

impl LayerList {
    pub fn new(layout_tree: LayoutTree) -> LayerList {
        let mut layer_list = LayerList {
            layout_tree,
            layers: RefCell::new(vec![]),
            next_layer_id: RefCell::new(0),
        };

        layer_list.generate_layers();
        layer_list
    }

    // fn get_element_by_id(&self, id: NodeId) -> Option<&LayoutElementNode> {
    //     let layout_tree = &self.layout_tree;
    //     let taffy_id = layout_tree.node_mapping.get(&id)?;
    //     let root = &layout_tree.root_layout_element;
    //     let element = self.get_element_by_taffy_id(root, *taffy_id)?;
    //     Some(element)
    // }

    fn new_layer(&self, order: isize) -> LayerId {
        let layer = Layer::new(self.next_layer_id(), order);
        let layer_id = layer.layer_id;
        self.layers.borrow_mut().push(layer);

        layer_id
    }

    fn get_layer(&self, layer_id: LayerId) -> Option<Ref<Layer>> {
        let layers = self.layers.borrow();
        let pos = layers.iter().position(|layer| layer.layer_id == layer_id)?;
        Some(Ref::map(layers, |layers| &layers[pos]))
    }

    fn get_layer_mut(&self, layer_id: LayerId) -> Option<RefMut<Layer>> {
        let layers = self.layers.borrow_mut();
        let pos = layers.iter().position(|layer| layer.layer_id == layer_id)?;
        Some(RefMut::map(layers, |layers| &mut layers[pos]))
    }

    fn generate_layers(&mut self) {
        self.layers.borrow_mut().clear();

        let root_id = self.layout_tree.root_id;
        let default_layer_id = self.new_layer(0);

        self.traverse(default_layer_id, root_id);
    }

    fn traverse(&self, layer_id: LayerId, layout_element_node_id: LayoutElementId) {
        let Some(layout_element) = self.layout_tree.get_node_by_id(layout_element_node_id) else {
            return;
        };

        let is_image = {
            let dom_node = self
                .layout_tree
                .render_tree
                .doc
                .get_node_by_id(layout_element.dom_node_id)
                .unwrap()
                ;
            match dom_node.node_type {
                crate::document::node::NodeType::Element(ref element_data) => {
                    element_data.tag_name.eq_ignore_ascii_case("img")
                },
                _ => false
            }
        };

        if is_image {
            let image_layer_id = self.new_layer(1);
            let mut image_layer = self.get_layer_mut(image_layer_id).unwrap();
            image_layer.add_element(layout_element.id);
        } else {
            let mut layer = self.get_layer_mut(layer_id).unwrap();
            layer.add_element(layout_element.id);
        }

        for child_id in layout_element.children.iter() {
            self.traverse(layer_id, *child_id);
        }
    }

    fn next_layer_id(&self) -> LayerId {
        let id = self.next_layer_id.borrow().clone();

        let mut next_layer_id = self.next_layer_id.borrow_mut();
        *next_layer_id += 1;

        id
    }
}