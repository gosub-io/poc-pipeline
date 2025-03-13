use vello::kurbo;
use vello::kurbo::{Affine, Rect, RoundedRect, Shape};
use vello::peniko::{Fill};
use crate::painter::commands::border::BorderStyle;
use crate::painter::commands::rectangle::Rectangle;
use crate::rasterizer::vello::brush::set_brush;
use crate::tiler::Tile;

pub(crate) fn do_paint_rectangle(scene: &mut vello::Scene, _tile: &Tile, rect: &Rectangle) {
    // // Translate the context to the tile's position and clip it.
    // cr.translate(-tile.rect.x, -tile.rect.y);
    // cr.rectangle(tile.rect.x, tile.rect.y, tile.rect.width, tile.rect.height);
    // cr.clip();

    // Draw background (if any background brush is defined)
    match rect.background() {
        Some(brush) => {
            let vello_rect = setup_rectangle_path(rect);
            let vello_brush = set_brush(brush, rect.rect());

            let vello_shape = vello_rect.get_shape();
            scene.fill(
                Fill::NonZero,
                Affine::IDENTITY,
                &vello_brush,
                None,
                &vello_shape,
            );
        }
        None => {}
    }

    // Create border
    match rect.border().style() {
        BorderStyle::None => {},
        BorderStyle::Solid => draw_single_border(scene, rect, vec![]),
        BorderStyle::Dashed => draw_single_border(scene, rect, vec![50.0, 10.0, 10.0, 10.0]),
        BorderStyle::Dotted => draw_single_border(scene, rect, vec![10.0, 10.0]),
        BorderStyle::Double => draw_double_border(scene, rect),
        BorderStyle::Groove => { unimplemented!() }
        BorderStyle::Ridge => { unimplemented!() }
        BorderStyle::Inset => { unimplemented!() }
        BorderStyle::Outset => { unimplemented!() }
        BorderStyle::Hidden => {
            // Don't display anything. But the border still takes up space. This is already
            // calculated in the box model by the layouter.
        }
    }
}

fn draw_single_border(scene: &mut vello::Scene, rect: &Rectangle, dashes: Vec<f64>) {
    let vello_shape = setup_rectangle_path(rect);
    let vello_brush = set_brush(&rect.border().brush(), rect.rect());
    let vello_stroke = kurbo::Stroke::new(rect.border().width() as f64).with_dashes(0.0, dashes);

    scene.stroke(
        &vello_stroke,
        Affine::IDENTITY,
        &vello_brush,
        None,
        &vello_shape,
    );
}

fn draw_double_border(scene: &mut vello::Scene, rect: &Rectangle) {
    let vello_shape = setup_rectangle_path(rect);
    let vello_brush = set_brush(&rect.border().brush(), rect.rect());

    if rect.border().width() < 3.0 {
        // When the width is less than 3.0, we just draw a single line as there is no room for
        // a double border
        scene.stroke(
            &kurbo::Stroke::new(rect.border().width() as f64),
            Affine::IDENTITY,
            &vello_brush,
            None,
            &vello_shape,
        );

        return;
    }

    // The formula: outer border: (N-1) / 2, 1px gap, inner border: (N-1) / 2

    // Outer border
    let width = (rect.border().width() / 2.0).floor();
    scene.stroke(
        &kurbo::Stroke::new(width as f64),
        Affine::IDENTITY,
        &vello_brush,
        None,
        &vello_shape,
    );

    let gap_size = 1.0;

    // inner border
    let inner_border_rect = Rect::new(
        rect.rect().x + width as f64 + gap_size,
        rect.rect().y + width as f64 + gap_size,
        rect.rect().width - width as f64 - gap_size,
        rect.rect().height - width as f64 - gap_size
    );
    scene.stroke(
        &kurbo::Stroke::new(width as f64),
        Affine::IDENTITY,
        &vello_brush,
        None,
        &inner_border_rect,
    );
}

enum RectType {
    RoundedRect(RoundedRect),
    Rect(Rect),
}

impl RectType {
    pub(crate) fn get_shape(&self) -> &impl Shape {
        match self {
            RectType::RoundedRect(r) => r,
            RectType::Rect(r) => r,
        }
    }
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