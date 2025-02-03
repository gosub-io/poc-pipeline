use std::collections::HashMap;
use ::taffy::{NodeId as TaffyNodeId, TaffyTree};
use ::taffy::prelude::{length, TaffyMaxContent};
use crate::layouter::boxmodel::BoxModel;
use crate::render_tree::RenderTree;
use crate::document::node::NodeId as DomNodeId;
use crate::layouter::taffy::generate_with_taffy;

pub mod taffy;
mod text;
mod boxmodel;

pub(crate) struct ViewportSize {
    pub(crate) width: f64,
    pub(crate) height: f64,
}

#[derive(Debug)]
pub struct LayoutElementNode {
    // Id of the node in the DOM, contains the data, like element name, attributes, etc.
    pub dom_node_id: DomNodeId,
    // Id of the node in the layout tree. Contains all layout information
    pub taffy_node_id: TaffyNodeId,
    // Children of this node
    pub children: Vec<LayoutElementNode>,
    // Generated boxmodel for this node
    pub box_model: Option<BoxModel>,
}

pub(crate) struct LayoutTree {
    // Wrapped render tree
    render_tree: RenderTree,
    /// Generated layout tree
    pub taffy_tree: TaffyTree,
    /// Root Taffy ID of the element in the tree
    pub taffy_root_id: TaffyNodeId,
    /// List of all elements in the layout tree
    pub root_layout_element: LayoutElementNode,
    // Mapping of node IDs between the Taffy and DOM trees
    pub node_mapping: HashMap<TaffyNodeId, DomNodeId>,
}

impl std::fmt::Debug for LayoutTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LayoutTree")
            // .field("taffy_tree", &self.taffy_tree)
            .field("taffy_root_id", &self.taffy_root_id)
            .field("root_layout_element", &self.root_layout_element)
            .field("node_mapping", &self.node_mapping)
            .finish()
    }
}

pub fn generate_layout(render_tree: RenderTree, viewport: ViewportSize) -> LayoutTree {
    let layout_tree = generate_with_taffy(render_tree, viewport);
    dbg!(&layout_tree);
    layout_tree
}