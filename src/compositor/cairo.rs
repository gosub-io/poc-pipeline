use gtk4::cairo;
use crate::compositor::cairo::compositor::cairo_compositor;
use crate::compositor::Composable;
use crate::layering::layer::LayerId;

pub struct CairoCompositorConfig {
    pub cr: cairo::Context,
}

mod compositor;

pub struct CairoCompositor {}

impl Composable for CairoCompositor {
    type Config = CairoCompositorConfig;

    fn compose(config: Self::Config) {
        cairo_compositor(&config.cr, vec![LayerId::new(0), LayerId::new(1)]);
    }
}
