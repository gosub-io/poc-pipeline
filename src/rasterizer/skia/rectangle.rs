use skia_safe::{Paint, Vector};
use crate::common::geo::Rect;
use crate::painter::commands::border::BorderStyle;
use crate::painter::commands::rectangle::Rectangle;
use crate::rasterizer::skia::paint::set_paint;
use crate::tiler::Tile;

pub(crate) fn do_paint_rectangle(canvas: &skia_safe::Canvas, _tile: &Tile, rect: &Rectangle) {
    // Draw background (if any background brush is defined)
    match rect.background() {
        Some(brush) => {
            let shape = setup_rectangle_path(rect);
            let skia_paint = set_paint(brush, rect.rect());

            draw_rect(shape, canvas, skia_paint);
        }
        None => {}
    }

    // Create border
    match rect.border().style() {
        BorderStyle::None => {},
        BorderStyle::Solid => draw_single_border(canvas, rect, vec![]),
        BorderStyle::Dashed => draw_single_border(canvas, rect, vec![50.0, 10.0, 10.0, 10.0]),
        BorderStyle::Dotted => draw_single_border(canvas, rect, vec![10.0, 10.0]),
        BorderStyle::Double => draw_double_border(canvas, rect, vec![]),
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

fn draw_single_border(canvas: &skia_safe::Canvas, rect: &Rectangle, dashes: Vec<f64>) {
    let dashes = dashes.iter().map(|x| *x as f32).collect::<Vec<f32>>();

    let shape = setup_rectangle_path(rect);

    let mut skia_paint = set_paint(&rect.border().brush(), rect.rect());
    skia_paint.set_stroke(true);
    skia_paint.set_stroke_width(rect.border().width());
    skia_paint.set_stroke_cap(skia_safe::PaintCap::Round);
    skia_paint.set_path_effect(skia_safe::PathEffect::dash(&dashes, 0.0));

    draw_rect(shape, canvas, skia_paint);
}

fn draw_double_border(canvas: &skia_safe::Canvas, rect: &Rectangle, dashes: Vec<f64>) {
    let dashes = dashes.iter().map(|x| *x as f32).collect::<Vec<f32>>();

    let shape = setup_rectangle_path(rect);

    let mut skia_paint = set_paint(&rect.border().brush(), rect.rect());
    skia_paint.set_stroke(true);
    skia_paint.set_stroke_width(rect.border().width());
    skia_paint.set_stroke_cap(skia_safe::PaintCap::Round);
    skia_paint.set_path_effect(skia_safe::PathEffect::dash(&dashes, 0.0));

    if rect.border().width() < 3.0 {
        // When the width is less than 3.0, we just draw a single line as there is no room for
        // a double border
        draw_rect(shape, canvas, skia_paint);

        return;
    }

    // The formula: outer border: (N-1) / 2, 1px gap, inner border: (N-1) / 2

    // Outer border
    let width = (rect.border().width() / 2.0).floor();
    skia_paint.set_stroke_width(width);
    draw_rect(shape, canvas, skia_paint);

    let gap_size = 1.0;

    // inner border
    let inner_border_rect = Rect::new(
        rect.rect().x + width as f64 + gap_size,
        rect.rect().y + width as f64 + gap_size,
        rect.rect().width - width as f64 - gap_size,
        rect.rect().height - width as f64 - gap_size
    );
    let shape = setup_rectangle_path(rect);
    let skia_paint = set_paint(&rect.border().brush(), inner_border_rect);
    draw_rect(shape, canvas, skia_paint);
}

fn draw_rect(shape: ShapeEnum, canvas: &skia_safe::Canvas, paint: Paint) {
    match shape {
        ShapeEnum::Rect(rect) => {
            canvas.draw_rect(&rect, &paint);
        }
        ShapeEnum::RoundedRect(rounded_rect) => {
            canvas.draw_rrect(&rounded_rect, &paint);
        }
    }
}

enum ShapeEnum {
    Rect(skia_safe::Rect),
    RoundedRect(skia_safe::RRect),
}

fn setup_rectangle_path(rect: &Rectangle) -> ShapeEnum {
    if rect.is_rounded() {
        let (r_tl, r_tr, r_br, r_bl) = rect.radius();

        return ShapeEnum::RoundedRect(skia_safe::RRect::new_rect_radii(
            skia_safe::Rect::new(
                rect.rect().x as f32,
                rect.rect().y as f32,
                (rect.rect().x + rect.rect().width) as f32,
                (rect.rect().y + rect.rect().height) as f32,
            ),
            &[
                Vector::new(r_tl.x as f32, r_tl.y as f32),
                Vector::new(r_tr.x as f32, r_tr.y as f32),
                Vector::new(r_br.x as f32, r_br.y as f32),
                Vector::new(r_bl.x as f32, r_bl.y as f32)
            ],
        ))
    }

    ShapeEnum::Rect(skia_safe::Rect::new(
        rect.rect().x as f32,
        rect.rect().y as f32,
        (rect.rect().x + rect.rect().width) as f32,
        (rect.rect().y + rect.rect().height) as f32,
    ))
}