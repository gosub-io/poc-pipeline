use resvg::usvg;

#[derive(Clone)]
pub struct Svg {
    pub tree: usvg::Tree,
}

impl Svg {
    #[allow(unused)]
    pub fn new(tree: usvg::Tree) -> Svg {
        Svg {
            tree,
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
