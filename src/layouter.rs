use ::taffy::{BoxSizing, Display, NodeId, Rect, Size, Style, TaffyTree};
use ::taffy::prelude::{length, TaffyMaxContent};
use crate::layouter::taffy::generate_taffy_tree;
use crate::render_tree::RenderTree;

pub(crate) mod taffy;
mod text;
mod boxmodel;

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

        /*
    let mut tree: TaffyTree<()> = TaffyTree::new();

    let node1 = tree.new_leaf(
        Style {
            box_sizing: BoxSizing::BorderBox,
            size: Size { width: length(50.0), height: length(50.0) },
            margin: Rect { top: length(10.0), left: length(10.0), right: length(10.0), bottom: length(10.0),},
            padding: Rect { top: length(10.0), left: length(10.0), right: length(10.0), bottom: length(10.0),},
            border: Rect { top: length(10.0), left: length(10.0), right: length(10.0), bottom: length(10.0),},
            display: Display::Block,
            ..Default::default()
        },
    ).unwrap();

    let node2 = tree.new_leaf(
        Style {
            box_sizing: BoxSizing::BorderBox,
            size: Size { width: length(50.0), height: length(50.0) },
            // margin: Rect { top: length(10.0), left: length(10.0), right: length(10.0), bottom: length(10.0),},
            padding: Rect { top: length(10.0), left: length(10.0), right: length(10.0), bottom: length(10.0),},
            border: Rect { top: length(10.0), left: length(10.0), right: length(10.0), bottom: length(10.0),},
            display: Display::Block,
            ..Default::default()
        },
    ).unwrap();

    let node3 = tree.new_leaf(
        Style {
            box_sizing: BoxSizing::BorderBox,
            size: Size { width: length(50.0), height: length(50.0) },
            margin: Rect { top: length(10.0), left: length(10.0), right: length(10.0), bottom: length(10.0),},
            // padding: Rect { top: length(10.0), left: length(10.0), right: length(10.0), bottom: length(10.0),},
            border: Rect { top: length(10.0), left: length(10.0), right: length(10.0), bottom: length(10.0),},
            display: Display::Block,
            ..Default::default()
        },
    ).unwrap();


    let root_node = tree.new_with_children(
        Style {
            box_sizing: BoxSizing::BorderBox,
            size: Size { width: length(500.0), height: length(500.0) },
            display: Display::Block,
            ..Default::default()
        },
        &[node1, node2, node3],
    ).unwrap();

    tree.compute_layout(root_node, Size::MAX_CONTENT).unwrap();

    LayoutTree {
        render_tree: render_tree,
        taffy_tree: tree,
        taffy_root_id: root_node,
    }
         */
}