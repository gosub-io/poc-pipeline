use gtk4::cairo::{Context, Error, Format, ImageSurface};
use gtk4::pango::{FontDescription, SCALE, Layout};
use pangocairo::functions::create_layout;
use crate::common::font::pango::{find_available_font, to_pango_weight};

/// Retrieves the pango layout for the given text, font family, font size and maximum width.
/// it will wrap any long lines based on the pixels found in width.
pub fn get_text_layout(text: &str, font_family: &str, font_size: f64, font_weight: usize, width: f64) -> Result<Layout, Error> {
    let surface = ImageSurface::create(Format::ARgb32, 1, 1)?;
    let cr = Context::new(&surface)?;
    let layout = create_layout(&cr);

    let selected_family = find_available_font(font_family, &layout.context());
    let mut font_desc = FontDescription::new();
    font_desc.set_family(&selected_family);
    font_desc.set_size((font_size / 1.3 * SCALE as f64) as i32);
    font_desc.set_weight(to_pango_weight(font_weight));
    layout.set_font_description(Some(&font_desc));

    layout.set_text(text);
    layout.set_width((width * SCALE as f64) as i32);
    layout.set_wrap(gtk4::pango::WrapMode::Word);
    // layout.set_wrap(gtk4::pango::WrapMode::Char);

    Ok(layout)
}