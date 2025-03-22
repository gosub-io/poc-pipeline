use crate::layouter::text::Alignment;
use skia_safe::Paint;
use skia_safe::textlayout::{Paragraph, ParagraphBuilder, ParagraphStyle, TextStyle};

thread_local! {
    static FC: skia_safe::textlayout::FontCollection = {
        let mut fc = skia_safe::textlayout::FontCollection::new();
        fc.set_default_font_manager(skia_safe::FontMgr::new(), None);
        fc
    };
}

pub fn get_skia_paragraph(text: &str, font_family: &str, font_size: f64, line_height: f64, max_width: f64, _alignment: Alignment) -> Paragraph {
    let paragraph_style = ParagraphStyle::new();
    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, FC.with(|fc| fc.clone()));

    let mut paint = Paint::default();
    paint.set_anti_alias(false);

    let mut ts = TextStyle::new();
    ts.set_foreground_paint(&paint);
    ts.set_font_size(font_size as f32);
    ts.set_font_families(&[font_family]);
    ts.set_height(line_height as f32);

    paragraph_builder.push_style(&ts);
    paragraph_builder.add_text(text);

    let mut paragraph = paragraph_builder.build();
    paragraph.layout(max_width as f32);

    paragraph
}