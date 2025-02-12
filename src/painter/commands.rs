use crate::painter::commands::rectangle::Rectangle;
use crate::painter::commands::text::Text;

mod color;
mod text;
mod image;
mod border;
mod rectangle;
mod brush;

#[derive(Clone, Debug)]
struct Trbl<T> {
    top: T,
    right: T,
    bottom: T,
    left: T,
}

#[derive(Clone, Debug)]
enum PaintCommands {
    Text(Text),
    Rectangle(Rectangle),
}

impl PaintCommands {
    pub fn text(text: Text) -> Self {
        PaintCommands::Text(text)
    }

    pub fn rectangle(rectangle: Rectangle) -> Self {
        PaintCommands::Rectangle(rectangle)
    }
}