use std::sync::{Mutex, OnceLock};
use parley::Layout;

static FONT_CTX: OnceLock<Mutex<parley::FontContext>> = OnceLock::new();
static LAYOUT_CTX: OnceLock<Mutex<parley::LayoutContext>> = OnceLock::new();
#[derive(Clone, Copy, Debug, PartialEq)]

pub struct ColorBrush {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Default for ColorBrush {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

pub fn get_font_context() -> std::sync::MutexGuard<'static, parley::FontContext> {
    FONT_CTX
        .get_or_init(|| Mutex::new(parley::FontContext::new()))
        .lock()
        .unwrap()
}

fn get_layout_context() -> std::sync::MutexGuard<'static, parley::LayoutContext> {
    LAYOUT_CTX
        .get_or_init(|| Mutex::new(parley::LayoutContext::new()))
        .lock()
        .unwrap()
}

pub fn get_text_layout(text: &str, font_family: &str, font_size: f64) -> Layout<ColorBrush> {
    let font_stack = parley::FontStack::from(font_family);

    let display_scale = 1.0;
    let max_advance = Some(400.0 * display_scale);

    let mut font_ctx = parley::FontContext::new();
    let mut layout_ctx = parley::LayoutContext::new();

    let mut builder = layout_ctx.ranged_builder(&mut font_ctx, text, display_scale);
    builder.push_default(font_stack);
    builder.push_default(parley::StyleProperty::LineHeight(1.0));
    builder.push_default(parley::StyleProperty::FontSize(font_size as f32));

    let mut layout: Layout<ColorBrush> = builder.build(text);
    layout.break_all_lines(max_advance);
    layout.align(max_advance, parley::layout::Alignment::Start);

    layout
}

pub fn get_text_dimension(text: &str, font_family: &str, font_size: f64) -> (f64, f64) {
    let layout = get_text_layout(text, font_family, font_size);
    (layout.width() as f64, layout.height() as f64)
}