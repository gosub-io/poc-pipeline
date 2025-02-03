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

pub type LayoutElementId = usize;

#[derive(Debug, Clone)]
pub struct LayoutElementNode {
    pub id: LayoutElementId,
    // Id of the node in the DOM, contains the data, like element name, attributes, etc.
    pub dom_node_id: DomNodeId,
    // Id of the node in the layout tree. Contains all layout information
    pub taffy_node_id: TaffyNodeId,
    // Children of this node
    pub children: Vec<LayoutElementId>,
    // Generated boxmodel for this node
    pub box_model: BoxModel,
}


#[derive(Debug, Clone)]
pub struct TaffyStruct {
    pub tree: TaffyTree,
    pub root_id: TaffyNodeId,
}

#[derive(Clone)]
pub struct LayoutTree {
    // Wrapped render tree
    pub render_tree: RenderTree,
    // Wrapped taffy tree
    pub taffy: TaffyStruct,

    pub arena : HashMap<LayoutElementId, LayoutElementNode>,
    pub root_id: LayoutElementId,
}

impl LayoutTree {
    pub fn get_node_by_id(&self, node_id: LayoutElementId) -> Option<&LayoutElementNode> {
        self.arena.get(&node_id)
    }
}

impl std::fmt::Debug for LayoutTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LayoutTree")
            .field("arena", &self.arena)
            .field("root_id", &self.root_id)
            .finish()
    }
}

pub fn generate_layout(render_tree: RenderTree, viewport: ViewportSize) -> LayoutTree {
    generate_with_taffy(render_tree, viewport)
}