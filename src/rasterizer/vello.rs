use crate::rasterizer::Rasterable;
use crate::rasterizer::texture::TextureId;
use crate::tiler::Tile;

pub struct VelloRasterizer {}

impl Rasterable for VelloRasterizer {
    // Rasterize the given tile with a new texture
    fn rasterize(tile: &Tile) -> TextureId {
        unimplemented!()
    }
}