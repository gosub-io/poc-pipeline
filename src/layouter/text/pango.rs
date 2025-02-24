use gtk4::cairo::{Context, Error, Format, ImageSurface};
use gtk4::pango::{FontDescription, SCALE, Layout};
use pangocairo::functions::create_layout;
use pangocairo::pango::WrapMode;
use crate::common;
use crate::common::document::style::{StyleProperty, StylePropertyList, StyleValue, TextWrap};
use crate::common::font::pango::{find_available_font, to_pango_weight};

/// Retrieves the pango layout for the given text, font family, font size and maximum width.
/// it will wrap any long lines based on the pixels found in width.
pub fn get_text_layout(text: &str, font_family: &str, font_size: f64, font_weight: usize, width: f64, style: &StylePropertyList) -> Result<Layout, Error> {
    let surface = ImageSurface::create(Format::ARgb32, 1, 1)?;
    let cr = Context::new(&surface)?;
    let layout = create_layout(&cr);

    let selected_family = find_available_font(font_family, &layout.context());
    let mut font_desc = FontDescription::new();
    font_desc.set_family(&selected_family);
    font_desc.set_size((font_size * SCALE as f64) as i32);
    font_desc.set_weight(to_pango_weight(font_weight));
    layout.set_font_description(Some(&font_desc));

    // dbg!(style.get_property(StyleProperty::LineHeight));
    // match style.get_property(StyleProperty::LineHeight) {
    //     Some(StyleValue::Unit(line_height, _)) => {
    //         layout.set_spacing((line_height * SCALE as f32) as i32);
    //     },
    //     Some(StyleValue::Number(line_height)) => {
    //         layout.set_spacing((line_height * SCALE as f32) as i32);
    //     },
    //     Some(StyleValue::Percentage(line_height)) => {
    //         layout.set_spacing((font_size * *line_height as f64 / 100.0 * SCALE as f64) as i32);
    //     },
    //     _ => {},
    // }

    layout.set_text(text);
    layout.set_width((width * SCALE as f64) as i32);

    // From style
    match style.get_property(StyleProperty::TextWrap) {
        Some(StyleValue::TextWrap(wrap)) => {
            match wrap {
                TextWrap::Wrap => layout.set_wrap(WrapMode::Word),
                TextWrap::NoWrap => {}, // What to do here?
                TextWrap::Balance => {}, // What to do here?
                TextWrap::Pretty => {}, // What to do here?
                TextWrap::Stable => {}, // What to do here?
                TextWrap::Initial => {}, // What to do here?
                TextWrap::Inherit => {}, // What to do here?
                TextWrap::Revert => {}, // What to do here?
                TextWrap::RevertLayer => {}, // What to do here?
                TextWrap::Unset => {}, // What to do here?
            }
        },
        _ => layout.set_wrap(WrapMode::Word),
    }

    Ok(layout)
}