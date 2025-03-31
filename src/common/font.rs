#[cfg(not(any(feature = "text_parley", feature = "text_pango", feature = "text_skia")))]
compile_error!("Either the 'text_parley' 'text_skia' or 'text_pango' feature must be enabled");

#[cfg(feature = "text_parley")]
pub mod parley;
#[cfg(feature = "text_pango")]
pub mod pango;
#[cfg(feature = "text_skia")]
pub mod skia;

#[derive(Debug, Clone)]
pub enum FontAlignment {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Debug, Clone)]
pub struct FontInfo {
    pub family: String,
    pub size: f64,
    pub weight: i32,
    pub width: i32,
    pub slant: i32,
    pub line_height: f64,
    pub alignment: FontAlignment,
}