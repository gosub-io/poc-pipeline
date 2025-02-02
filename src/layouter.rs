use crate::layouter::taffy::generate_taffy_tree;
use crate::render_tree::RenderTree;

mod taffy;
mod text;

pub(crate) struct ViewportSize {
    pub(crate) width: f64,
    pub(crate) height: f64,
}

pub(crate) struct Layouter {
    render_tree: RenderTree,
}

impl Layouter {
    pub fn new(render_tree: RenderTree) -> Layouter {
        Layouter {
            render_tree,
        }
    }

    pub fn generate(&mut self, viewport: ViewportSize) {
        let taffy_tree = generate_taffy_tree(&self.render_tree, viewport);
    }
}

