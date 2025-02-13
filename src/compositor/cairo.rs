use gtk4::cairo;
use crate::compositor::cairo::compositor::cairo_compositor;
use crate::compositor::Composable;

pub struct CairoCompositorConfig {
    pub cr: cairo::Context,
}

mod compositor;

pub struct CairoCompositor {}

impl Composable for CairoCompositor {
    type Config = CairoCompositorConfig;

    fn compositor(config: Self::Config) {
        cairo_compositor(&config.cr);
    }
}
