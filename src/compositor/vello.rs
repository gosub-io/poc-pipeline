use crate::common::browser_state::get_browser_state;
use crate::compositor::Composable;
use crate::compositor::vello::compositor::vello_compositor;
use crate::layering::layer::LayerId;

pub struct VelloCompositorConfig {
    scene: vello::Scene,
}

mod compositor;

pub struct VelloCompositor {}

impl Composable for VelloCompositor {
    type Config = VelloCompositorConfig;

    fn compose(mut config: Self::Config) {
        let binding = get_browser_state();
        let state = binding.read().expect("Failed to get browser state");

        let mut layers = vec![];
        if state.visible_layer_list[0] {
            layers.push(LayerId::new(0));
        }
        if state.visible_layer_list[1] {
            layers.push(LayerId::new(1));
        }

        vello_compositor(&mut config.scene, layers);
    }
}
