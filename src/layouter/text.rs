#[cfg(not(any(feature = "parley", feature = "pango")))]
compile_error!("Either the 'parley' or 'pango' feature must be enabled");

#[cfg(all(feature = "parley", feature = "pango"))]
compile_error!("Only one of the 'parley' or 'pango' features can be enabled");

#[cfg(feature = "parley")]
pub mod parley;
#[cfg(feature = "parley")]
pub use crate::layouter::text::parley::get_text_layout;

#[cfg(feature = "pango")]
pub mod pango;
#[cfg(feature = "pango")]
pub use crate::layouter::text::pango::get_text_layout;


/// Text alignment
#[derive(Clone, Debug, Copy)]
pub enum Alignment {
    /// Alignment of text is at the start (depends on LTR)
    Start,
    /// Alignment of text is at the end (depends on LTR)
    End,
    /// Alignment is centered
    Middle,
    /// alignment is justified (full column width)
    Justified,
}