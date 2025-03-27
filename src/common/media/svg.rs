use resvg::usvg;
use crate::common::geo::Dimension;
use crate::common::media::Image;

#[derive(Clone)]
pub struct Svg {
    pub tree: usvg::Tree,
    /// Rendered dimension of the rendered image
    pub rendered_dimension: Dimension,
    /// Rendered image in the given dimension
    pub rendered_image: Image,
}

impl Svg {
    #[allow(unused)]
    pub fn new(tree: usvg::Tree) -> Svg {
        Svg {
            tree,
            rendered_dimension: Dimension::ZERO,
            rendered_image: Image::new(0, 0),
        }
    }
}

impl std::fmt::Debug for Svg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Svg")
            .field("tree", &self.tree)
            .finish()
    }
}
