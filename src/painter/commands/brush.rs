use crate::painter::commands::color::Color;
use crate::painter::commands::image::Image;

#[derive(Clone, Debug)]
pub enum Brush {
    Solid(Color),
    Image(Image),
    // Gradient(Gradient),
}

impl Brush {
    pub fn solid(color: Color) -> Self {
        Brush::Solid(color)
    }

    pub fn image(data: Vec<u8>, width: u32, height: u32) -> Self {
        Brush::Image(Image::new(data, width, height))
    }

    // pub fn gradient(gradient: Gradient) -> Self {
    //     Brush::Gradient(gradient)
    // }
}
