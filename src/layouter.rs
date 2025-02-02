use ::taffy::{NodeId, TaffyTree};
use crate::layouter::taffy::generate_taffy_tree;
use crate::render_tree::RenderTree;

mod taffy;
mod text;

pub(crate) struct ViewportSize {
    pub(crate) width: f64,
    pub(crate) height: f64,
}

pub(crate) struct LayoutTree {
    // Wrapped render tree
    render_tree: RenderTree,
    /// Generated layout tree
    pub taffy_tree: TaffyTree,
    /// Root Taffy ID of the element in the tree
    pub taffy_root_id: NodeId,
}

pub fn generate_layout(render_tree: RenderTree, viewport: ViewportSize) -> LayoutTree {
    let (tree, root_id) = generate_taffy_tree(&render_tree, viewport);

    LayoutTree {
        render_tree: render_tree,
        taffy_tree: tree,
        taffy_root_id: root_id,
    }
}