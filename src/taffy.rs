use taffy::prelude::*;

fn main() {
    let mut tree: TaffyTree<()> = TaffyTree::new();

    let node1 = tree.new_leaf(
    Style {
            size: Size { width: length(100.0), height: length(100.0) },
            margin: Rect { top: length(10.0), left: length(10.0), right: length(10.0), bottom: length(10.0),},
            padding: Rect { top: length(10.0), left: length(10.0), right: length(10.0), bottom: length(10.0),},
            display: Display::Block,
            ..Default::default()
        },
    ).unwrap();

    let node2 = tree.new_leaf(
        Style {
            size: Size { width: length(100.0), height: length(100.0) },
            // margin: Rect { top: length(10.0), left: length(10.0), right: length(10.0), bottom: length(10.0),},
            padding: Rect { top: length(10.0), left: length(10.0), right: length(10.0), bottom: length(10.0),},
            display: Display::Block,
            ..Default::default()
        },
    ).unwrap();

    let node3 = tree.new_leaf(
        Style {
            size: Size { width: length(100.0), height: length(100.0) },
            margin: Rect { top: length(10.0), left: length(10.0), right: length(10.0), bottom: length(10.0),},
            // padding: Rect { top: length(10.0), left: length(10.0), right: length(10.0), bottom: length(10.0),},
            display: Display::Block,
            ..Default::default()
        },
    ).unwrap();


    let root_node = tree.new_with_children(
        Style {
                size: Size { width: length(500.0), height: length(500.0) },
                // margin: Rect { top: length(10.0), left: length(10.0), right: length(10.0), bottom: length(10.0),},
                display: Display::Block,
                ..Default::default()
            },
            &[node1, node2, node3],
        ).unwrap();

    // Call compute_layout on the root of your tree to run the layout algorithm
    tree.compute_layout(root_node, Size::MAX_CONTENT).unwrap();

    tree.print_tree(root_node);
}