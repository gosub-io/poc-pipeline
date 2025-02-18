use gtk4::cairo::{Context, Error, Format, ImageSurface};
use gtk4::pango::{FontDescription, SCALE, Layout};
use pangocairo::functions::create_layout;

/// Retrieves the pango layout for the given text, font family, font size and maximum width.
/// it will wrap any long lines based on the pixels found in width.
pub fn get_text_layout(text: &str, font_family: &str, font_size: f64, width: f64) -> Result<Layout, Error> {
    let surface = ImageSurface::create(Format::ARgb32, 1, 1)?;
    let cr = Context::new(&surface)?;
    let layout = create_layout(&cr);

    layout.set_text(text);

    let font = FontDescription::from_string(&format!("{} {}", font_family, font_size));
    layout.set_font_description(Some(&font));

    layout.set_width((width * SCALE as f64) as i32);
    layout.set_wrap(gtk4::pango::WrapMode::Word);
    // layout.set_wrap(gtk4::pango::WrapMode::Char);

    Ok(layout)
}

// pub fn get_text_dimension(text: &str, font_family: &str, font_size: f64, width: f64) -> Dimension {
//     match get_text_layout(text, font_family, font_size, width) {
//         Ok(layout) => Dimension::new(
//             (layout.width() / SCALE) as f64,
//             (layout.height() / SCALE)  as f64
//         ),
//         Err(_) => Dimension::ZERO,
//     }
// }