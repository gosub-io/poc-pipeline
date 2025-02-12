use crate::painter::commands::color::Color;
use crate::painter::commands::image::Image;

#[derive(Clone, Debug)]
pub enum Brush {
    Solid(Color),
    Image(Image),
    // Gradient(Gradient),
}
