#[cfg(not(any(feature = "parley", feature = "pango")))]
compile_error!("Either the 'parley' or 'pango' feature must be enabled");

#[cfg(all(feature = "parley", feature = "pango"))]
compile_error!("Only one of the 'parley' or 'pango' features can be enabled");

#[cfg(feature = "parley")]
pub mod parley;
#[cfg(feature = "pango")]
pub mod pango;