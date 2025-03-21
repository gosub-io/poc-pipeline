use crate::common::browser_state::get_browser_state;
use crate::compositor::Composable;
use crate::compositor::skia::compositor::skia_compositor;
use crate::layering::layer::LayerId;

pub struct SkiaCompositorConfig {}

mod compositor;

pub struct SkiaCompositor {}

impl Composable for SkiaCompositor {
    type Config = SkiaCompositorConfig;
    type Return = skia_safe::Surface;

    fn compose(_config: Self::Config) -> Self::Return {
        let binding = get_browser_state();
        let state = binding.read().expect("Failed to get browser state");

        let mut layers = vec![];
        if state.visible_layer_list[0] {
            layers.push(LayerId::new(0));
        }
        if state.visible_layer_list[1] {
            layers.push(LayerId::new(1));
        }

        // Compose the scene from the different layers we have selected
        skia_compositor(layers)
    }
}

