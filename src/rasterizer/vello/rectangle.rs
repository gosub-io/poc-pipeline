use vello::kurbo;
use vello::kurbo::{Affine, Rect, RoundedRect};
use vello::peniko::Fill;
use crate::painter::commands::border::BorderStyle;
use crate::painter::commands::rectangle::Rectangle;
use crate::rasterizer::vello::brush::set_brush;
use crate::tiler::Tile;

pub(crate) fn do_paint_rectangle(scene: &mut vello::Scene, tile: &Tile, rectangle: &Rectangle) {
    // // Translate the context to the tile's position and clip it.
    // cr.translate(-tile.rect.x, -tile.rect.y);
    // cr.rectangle(tile.rect.x, tile.rect.y, tile.rect.width, tile.rect.height);
    // cr.clip();

    // Create initial rect
    match rectangle.background() {
        Some(brush) => {
            let vello_rect = setup_rectangle_path(rectangle);
            let vello_brush = set_brush(brush, rectangle.rect());

            let s =

            scene.fill(
                Fill::NonZero,
                Affine::IDENTITY,
                vello_brush,
                None,
                &vello_rect,
            );
        }
        None => {}
    }

    // Create border
    let vello_rect = setup_rectangle_path(rectangle);
    let vello_brush = set_brush(&rectangle.border().brush(), rectangle.rect());

    match rectangle.border().style() {
        BorderStyle::None => {
            // No border
        }
        BorderStyle::Solid => {
            // Complete solid line
            scene.stroke(
                &kurbo::Stroke::new(rectangle.border().width() as f64).with_dashes(0.0, vec![]),
                Affine::IDENTITY,
                &vello_brush,
                None,
                &vello_rect,
            );
        }
        BorderStyle::Dashed => {
            // 50px dash, 10px gap, 10px dash, 10px gap
            scene.stroke(
                &kurbo::Stroke::new(rectangle.border().width() as f64).with_dashes(0.0, &[50.0, 10.0, 10.0, 10.0]),
                Affine::IDENTITY,
                &vello_brush,
                None,
                &vello_rect,
            );
        }
        BorderStyle::Dotted => {
            // 10px dash, 10px gap
            scene.stroke(
                &kurbo::Stroke::new(rectangle.border().width() as f64).with_dashes(0.0, &[10.0, 10.0]),
                Affine::IDENTITY,
                &vello_brush,
                None,
                &vello_rect,
            );
        }
        BorderStyle::Double => {
            if rectangle.border().width() >= 3.0 {
                // The formula  outer border: (N-1) / 2, 1px gap, inner border: (N-1) / 2

                // Outer border
                let width = (rectangle.border().width() / 2.0).floor();
                scene.stroke(
                    &kurbo::Stroke::new(width as f64),
                    Affine::IDENTITY,
                    &vello_brush,
                    None,
                    &vello_rect,
                );

                let gap_size = 1.0;

                // inner border
                let inner_border_rect = Rect::new(
                    rectangle.rect().x + width as f64 + gap_size,
                    rectangle.rect().y + width as f64 + gap_size,
                    rectangle.rect().width - width as f64 - gap_size,
                    rectangle.rect().height - width as f64 - gap_size
                );
                scene.stroke(
                    &kurbo::Stroke::new(width as f64),
                    Affine::IDENTITY,
                    &vello_brush,
                    None,
                    &inner_border_rect,
                );

            } else {
                // When the width is less than 3.0, we just draw a single line as there is no room for
                // a double border
                scene.stroke(
                    &kurbo::Stroke::new(rectangle.border().width() as f64),
                    Affine::IDENTITY,
                    &vello_brush,
                    None,
                    &vello_rect,
                );

            }
        }
        BorderStyle::Groove => {}
        BorderStyle::Ridge => {}
        BorderStyle::Inset => {}
        BorderStyle::Outset => {}
        BorderStyle::Hidden => {
            // Don't display anything. But the border still takes up space. This is already
            // calculated in the box model by the layouter.
        }
    }
}

enum RectType {
    RoundedRect(RoundedRect),
    Rect(Rect),
}

/// Creates a cairo rectangle with either sharp or rounded corners. Does not fill or stroke the path.
fn setup_rectangle_path(rect: &Rectangle) -> RectType {
    let (r_tl, r_tr, r_br, r_bl) = rect.radius();

    if r_tl == 0.0 && r_tr == 0.0 && r_br == 0.0 && r_bl == 0.0 {
        return RectType::Rect(Rect::new(
            rect.rect().x,
            rect.rect().y,
            rect.rect().width,
            rect.rect().height,
        ))
    }

    RectType::RoundedRect(RoundedRect::new(
        rect.rect().x,
        rect.rect().y,
        rect.rect().width,
        rect.rect().height,
        (r_tl, r_tr, r_br, r_bl)
    ))
}