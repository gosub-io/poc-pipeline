use std::collections::HashMap;
use std::ops::AddAssign;
use std::sync::{Arc, RwLock};
use crate::layouter::box_model::BoxModel;
use crate::rendertree_builder::{RenderTree, RenderNodeId};
use crate::common::document::node::{NodeId as DomNodeId};
use crate::common::geo::Dimension;
use crate::common::image::ImageId;

pub mod taffy;
pub mod text;
mod box_model;

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
pub struct ElementContextText {
    pub font_family: String,
    pub font_size: f64,
    pub text: String,
}

#[derive(Clone, Debug)]
pub struct ElementContextImage {
    pub src: String,
    pub image_id: ImageId,
    pub dimension: Dimension,
}

/// Information about the given element that is needed for different phases of the rendering pipeline. For instance,
/// image or text information.
#[derive(Debug, Clone)]
pub enum ElementContext {
    None,
    Text(ElementContextText),
    Image(ElementContextImage),
}

impl ElementContext {
    pub(crate) fn text(font_family: &str, font_size: f64, text: &str) -> ElementContext {
        Self::Text(ElementContextText{
            font_family: font_family.to_string(),
            font_size,
            text: text.to_string(),
        })
    }

    pub fn image(src: &str, image_id: ImageId, dimension: Dimension) -> ElementContext {
        Self::Image(ElementContextImage {
            src: src.to_string(),
            image_id,
            dimension,
        })
    }
}

#[derive(Debug, Clone)]
pub struct LayoutElementNode {
    pub id: LayoutElementId,
    /// Id of the node in the DOM, contains the data, like element name, attributes, etc.
    pub dom_node_id: DomNodeId,
    /// Id of the node in the render tree. This is normally the same node ID as the dom node ID
    pub render_node_id: RenderNodeId,
    /// Children of this node
    pub children: Vec<LayoutElementId>,
    /// Generated boxmodel for this node
    pub box_model: BoxModel,
    /// Element context. Used by different parts of the render engine
    pub context: ElementContext,
}

#[derive(Clone)]
pub struct LayoutTree {
    /// Wrapped render tree
    pub render_tree: RenderTree,
    /// Arena of layout nodes
    pub arena : HashMap<LayoutElementId, LayoutElementNode>,
    /// Root node of the layout tree
    pub root_id: LayoutElementId,
    /// Next node ID
    next_node_id: Arc<RwLock<LayoutElementId>>,
    // Root width and height
    pub root_dimension: Dimension,
}

impl LayoutTree {
    pub fn get_node_by_id(&self, node_id: LayoutElementId) -> Option<&LayoutElementNode> {
        self.arena.get(&node_id)
    }

    pub fn get_node_by_id_mut(&mut self, node_id: LayoutElementId) -> Option<&mut LayoutElementNode> {
        self.arena.get_mut(&node_id)
    }

    pub fn next_node_id(&self) -> LayoutElementId {
        let mut nid = self.next_node_id.write().unwrap();
        let id = *nid;
        *nid += 1;
        id
    }
}

impl std::fmt::Debug for LayoutTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LayoutTree")
            .field("arena", &self.arena)
            .field("root_id", &self.root_id)
            .field("root_dimension", &self.root_dimension)
            .finish()
    }
}

pub trait CanLayout {
    fn layout(&mut self, render_tree: RenderTree, viewport: crate::common::geo::Dimension) -> LayoutTree;
}