use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use crate::layouter::{LayoutElementId, LayoutTree};

pub type LayerId = usize;

#[derive(Clone)]
pub(crate) struct Layer {
    /// Layer ID
    pub layer_id: LayerId,
    /// Order of the layer
    #[allow(unused)]
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
    /// Wrapped layout tree
    pub layout_tree: LayoutTree,
    /// List of all (unique) layer IDs
    pub layer_ids: RefCell<Vec<LayerId>>,
    /// List of layers
    pub layers: RefCell<HashMap<LayerId, Layer>>,
    /// Next layer ID
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
            layers: RefCell::new(HashMap::new()),
            layer_ids: RefCell::new(Vec::new()),
            next_layer_id: RefCell::new(0),
        };

        layer_list.generate_layers();
        layer_list
    }

    /// Find the element at the given coordinates. It will return the given element if it is found or None otherwise
    pub fn find_element_at(&self, x: f64, y: f64) -> Option<LayoutElementId> {
        // This assumes that the layers are ordered from top to bottom
        for layer_id in self.layer_ids.borrow().iter().rev() {
            let binding = self.layers.borrow();
            let Some(layer) = binding.get(layer_id) else {
              continue;
            };

            for element_id in layer.elements.iter().rev() {
                let layout_element = self.layout_tree.get_node_by_id(*element_id).unwrap();
                let box_model = &layout_element.box_model;

                if x >= box_model.margin_box.x &&
                    x < box_model.margin_box.x + box_model.margin_box.width &&
                    y >= box_model.margin_box.y &&
                    y < box_model.margin_box.y + box_model.margin_box.height
                {
                    return Some(*element_id);
                }
            }
        }

        None
    }

    // Create a new layer to the list at the given order
    fn new_layer(&self, order: isize) -> LayerId {
        let layer = Layer::new(self.next_layer_id(), order);
        let layer_id = layer.layer_id;
        self.layer_ids.borrow_mut().push(layer_id);
        self.layers.borrow_mut().insert(layer_id, layer);

        layer_id
    }

    #[allow(unused)]
    fn get_layer(&self, layer_id: LayerId) -> Option<Ref<Layer>> {
        Ref::filter_map(self.layers.borrow(), |layers| layers.get(&layer_id)).ok()
    }

    fn get_layer_mut(&self, layer_id: LayerId) -> Option<RefMut<Layer>> {
        let layers = self.layers.borrow_mut();
        if layers.contains_key(&layer_id) {
            Some(RefMut::map(layers, |layers| layers.get_mut(&layer_id).unwrap()))
        } else {
            None
        }
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
        let mut next_id = self.next_layer_id.borrow_mut();
        let id = *next_id;
        *next_id += 1;

        id
    }
}