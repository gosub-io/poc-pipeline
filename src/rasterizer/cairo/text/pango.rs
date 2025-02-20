use gtk4::cairo::{Context, Error, Format, ImageSurface};
use gtk4::pango;
use gtk4::pango::{Weight, SCALE};
use pangocairo::functions::create_layout;
use pangocairo::pango::FontDescription;
use crate::painter::commands::text::Text;
use crate::rasterizer::cairo::brush::set_brush;
use crate::tiler::Tile;

pub(crate) fn do_paint_text(cr: &Context, tile: &Tile, cmd: &Text) -> Result<(), Error> {
    let surface = create_text_layout(cmd)?;

    // Save the context state. This allows us to do clipping and translation without worrying about
    // the state of the context.
    _ = cr.save()?;

    // Translate the context to the tile's position and clip it.
    cr.translate(-tile.rect.x, -tile.rect.y);
    cr.rectangle(tile.rect.x, tile.rect.y, tile.rect.width, tile.rect.height);
    cr.clip();

    cr.move_to(cmd.rect.x, cmd.rect.y);
    cr.set_source_surface(&surface, cmd.rect.x, cmd.rect.y)?;
    cr.paint()?;
    cr.restore()?;

    Ok(())
}

fn create_text_layout(cmd: &Text) -> Result<ImageSurface, Error> {
    let surface = ImageSurface::create(Format::ARgb32, cmd.rect.width as i32, cmd.rect.height as i32)?;
    let cr = Context::new(&surface)?;
    let layout = create_layout(&cr);

    let selected_family = find_available_font(cmd.font_family.as_str(), &layout.context());
    let mut font_desc = FontDescription::new();
    font_desc.set_family(&selected_family);
    // @TODO: Why do we need to divide by 1.3? It seems that this actually fills the layout correctly.
    font_desc.set_size(((cmd.font_size / 1.3) * SCALE as f64) as i32);
    font_desc.set_weight(to_pango_weight(cmd.font_weight));
    layout.set_font_description(Some(&font_desc));

    layout.set_text(cmd.text.as_str());
    layout.set_width((cmd.rect.width * SCALE as f64) as i32);
    layout.set_wrap(gtk4::pango::WrapMode::Word);
    // layout.set_wrap(gtk4::pango::WrapMode::Char);

    set_brush(&cr, &cmd.brush, cmd.rect.into());
    cr.move_to(0.0, 0.0);
    pangocairo::functions::show_layout(&cr, &layout);

    Ok(surface)
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


fn find_available_font(_families: &str, _ctx: &pango::Context) -> String {
    return "Ubuntu Sans".into();

/*
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
 */
}