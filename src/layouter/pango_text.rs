use gtk4::cairo::{Context, Error, Format, ImageSurface};
use gtk4::pango::{FontDescription, SCALE, Layout};
use pangocairo::functions::create_layout;

pub fn get_text_layout(text: &str, font_family: &str, font_size: f64, width: f64) -> Result<Layout, Error> {
    let surface = ImageSurface::create(Format::ARgb32, 1, 1)?;
    let cr = Context::new(&surface)?;
    let layout = create_layout(&cr);

    layout.set_text(text);
    let font = FontDescription::from_string(&format!("{} {}", font_family, font_size * SCALE as f64));
    layout.set_font_description(Some(&font));
    layout.set_width(width as i32 * SCALE);
    layout.set_wrap(gtk4::pango::WrapMode::Word);

    Ok(layout)
}

pub fn get_text_dimension(text: &str, font_family: &str, font_size: f64, width: f64) -> (f64, f64) {
    match get_text_layout(text, font_family, font_size, width) {
        Ok(layout) => {
            (layout.width() as f64, layout.height() as f64)
        }
        Err(_) => (0.0, 0.0),
    }
}