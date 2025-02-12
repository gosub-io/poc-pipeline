use gtk4::cairo::Context;
use crate::layouter::LayoutElementNode;

pub fn draw_wireframe(cr: &Context, el: &LayoutElementNode) {
    // Draw margin
    let m = el.box_model.margin_box;
    cr.set_source_rgba(1.0, 0.0, 0.0, 1.0);
    cr.rectangle(m.x, m.y, m.width, m.height);
    _ = cr.stroke();

    // Draw border
    let b = el.box_model.border_box();
    cr.set_source_rgba(48.0 / 255.0, 12.0 / 255.0, 124.0 / 255.0, 0.25);
    cr.rectangle(b.x, b.y, b.width, b.height);
    _ = cr.stroke();

    // Draw padding (blue)
    cr.set_source_rgba(173.0 / 255.0, 173.0 / 255.0, 247.0 / 255.0, 0.25);
    let p = el.box_model.padding_box();
    cr.rectangle(p.x, p.y, p.width, p.height);
    _ = cr.stroke();

    // Draw content (white fill with black stroke)
    let c = el.box_model.content_box();
    cr.set_source_rgba(173.0 / 255.0, 244.0 / 255.0, 247.0 / 255.0, 0.25);
    cr.rectangle(c.x, c.y, c.width, c.height);
    _ = cr.stroke();

    cr.rectangle(m.x, m.y, m.width, m.height);
    cr.set_source_rgba(1.0, 0.0, 0.0, 0.25);
    _ = cr.stroke();

}

pub fn draw_boxmodel(cr: &Context, el: &LayoutElementNode) {
    // Draw margin
    let m = el.box_model.margin_box;
    cr.set_source_rgba(243.0 / 255.0, 243.0 / 255.0, 173.0 / 255.0, 0.25);
    cr.rectangle(m.x, m.y, m.width, m.height);
    _ = cr.fill();

    // Draw border
    let b = el.box_model.border_box();
    cr.set_source_rgba(48.0 / 255.0, 12.0 / 255.0, 124.0 / 255.0, 0.25);
    cr.rectangle(b.x, b.y, b.width, b.height);
    _ = cr.fill();

    // Draw padding (blue)
    cr.set_source_rgba(173.0 / 255.0, 173.0 / 255.0, 247.0 / 255.0, 0.25);
    let p = el.box_model.padding_box();
    cr.rectangle(p.x, p.y, p.width, p.height);
    _ = cr.fill();

    // Draw content (white fill with black stroke)
    let c = el.box_model.content_box();
    cr.set_source_rgba(173.0 / 255.0, 244.0 / 255.0, 247.0 / 255.0, 0.25);
    cr.rectangle(c.x, c.y, c.width, c.height);
    _ = cr.fill();

    cr.rectangle(m.x, m.y, m.width, m.height);
    cr.set_source_rgba(1.0, 0.0, 0.0, 0.25);
    _ = cr.stroke();
}
