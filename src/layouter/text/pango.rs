use gtk4::cairo::{Context, Error, Format, ImageSurface};
use gtk4::pango::{FontDescription, SCALE, Layout, Weight};
use pangocairo::functions::create_layout;
use pangocairo::pango;
use pangocairo::prelude::{FontFamilyExt, FontMapExt};

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

fn to_pango_weight(w: usize) -> Weight {
    if w < 100 {
        Weight::Thin
    } else if w < 200 {
        Weight::Ultralight
    } else if w < 300 {
        Weight::Light
    } else if w < 400 {
        Weight::Normal
    } else if w < 500 {
        Weight::Medium
    } else if w < 600 {
        Weight::Semibold
    } else if w < 700 {
        Weight::Bold
    } else if w < 800 {
        Weight::Ultrabold
    } else {
        Weight::Heavy
    }
}

fn find_available_font(families: &str, ctx: &pango::Context) -> String {
    return "Ubuntu Sans".into();

    let available_fonts: Vec<String> = ctx.list_families().iter().map(|f| f.name().to_ascii_lowercase()).collect();

    for font in families.split(',') {
        if font == "system-ui" {
            continue;
        }

        println!("Checking for: {}", font);
        let font_name = font.trim().replace('"', ""); // Remove spaces & quotes

        if available_fonts.contains(&font_name.to_ascii_lowercase()) {
            return font_name; // Found a valid font!
        }
    }

    "FBserif".to_string() // Default fallback
}