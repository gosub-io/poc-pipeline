use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::AddAssign;
use std::rc::Rc;
use ::taffy::{NodeId as TaffyNodeId, TaffyTree};
use ::taffy::prelude::TaffyMaxContent;
use crate::layouter::boxmodel::BoxModel;
use crate::render_tree::{RenderTree, RenderNodeId};
use crate::document::node::{NodeId as DomNodeId, NodeId};
use crate::layouter::taffy::generate_with_taffy;

pub mod taffy;
pub mod text;
mod boxmodel;

pub(crate) struct ViewportSize {
    pub(crate) width: f64,
    pub(crate) height: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LayoutElementId(u64);

impl LayoutElementId {
    pub const fn new(val: u64) -> Self {
        Self(val)
    }
}

impl AddAssign<i32> for LayoutElementId {
    fn add_assign(&mut self, rhs: i32) {
        self.0 += rhs as u64;
    }
}

impl std::fmt::Display for LayoutElementId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "LayoutElementId({})", self.0)
    }
}


#[derive(Debug, Clone)]
pub struct LayoutElementNode {
    pub id: LayoutElementId,
    /// Id of the node in the DOM, contains the data, like element name, attributes, etc.
    pub dom_node_id: DomNodeId,
    /// Id of the node in the render tree. This is normally the same node ID as the dom node ID
    pub render_node_id: RenderNodeId,
    /// Id of the node in the layout tree. Contains all layout information
    pub taffy_node_id: TaffyNodeId,
    /// Children of this node
    pub children: Vec<LayoutElementId>,
    /// Generated boxmodel for this node
    pub box_model: BoxModel,
}


#[derive(Debug, Clone)]
pub struct TaffyStruct {
    pub tree: TaffyTree,
    pub root_id: TaffyNodeId,
}

#[derive(Clone)]
pub struct LayoutTree {
    /// Wrapped render tree
    pub render_tree: RenderTree,
    /// Wrapped taffy tree
    pub taffy: TaffyStruct,
    /// Arena of layout nodes
    pub arena : HashMap<LayoutElementId, LayoutElementNode>,
    /// Root node of the layout tree
    pub root_id: LayoutElementId,
    /// Next node ID
    next_node_id: Rc<RefCell<LayoutElementId>>,
}

impl LayoutTree {
    pub fn get_node_by_id(&self, node_id: LayoutElementId) -> Option<&LayoutElementNode> {
        self.arena.get(&node_id)
    }

    pub fn get_node_by_id_mut(&mut self, node_id: LayoutElementId) -> Option<&mut LayoutElementNode> {
        self.arena.get_mut(&node_id)
    }

    pub fn next_node_id(&self) -> LayoutElementId {
        let id = self.next_node_id.borrow().clone();

        let mut nid = self.next_node_id.borrow_mut();
        *nid += 1;

        id
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