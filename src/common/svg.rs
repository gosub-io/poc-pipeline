use std::ops::AddAssign;
use resvg::usvg;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SvgId(u64);

impl SvgId {
    pub const fn new(val: u64) -> Self {
        Self(val)
    }
}

impl AddAssign<i32> for SvgId {
    fn add_assign(&mut self, rhs: i32) {
        self.0 += rhs as u64;
    }
}

impl std::fmt::Display for SvgId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SvgId({})", self.0)
    }
}

#[derive(Clone)]
pub struct Svg {
    pub src: String,
    pub tree: usvg::Tree,
}

impl Svg {
    #[allow(unused)]
    pub fn new(src: &str, tree: usvg::Tree) -> Svg {

        Svg {
            src: src.to_string(),
            tree,
        }
    }
}

impl std::fmt::Debug for Svg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Image")
            .field("src", &self.src)
            .finish()
    }
}
