use gtk4::cairo::{Context, Error, Format, ImageSurface};
use gtk4::pango::SCALE;
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

    layout.set_text(cmd.text.as_str());
    let font_description = FontDescription::from_string(format!("{} {}", cmd.font_family.as_str(), cmd.font_size).as_str());
    layout.set_font_description(Some(&font_description));
    layout.set_width((cmd.rect.width * SCALE as f64) as i32);
    layout.set_wrap(gtk4::pango::WrapMode::Word);
    // layout.set_wrap(gtk4::pango::WrapMode::Char);

    set_brush(&cr, &cmd.brush, cmd.rect.into());
    cr.move_to(0.0, 0.0);
    pangocairo::functions::show_layout(&cr, &layout);

    Ok(surface)
}