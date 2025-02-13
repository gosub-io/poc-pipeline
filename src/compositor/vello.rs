pub struct VelloCompositorConfig {
    // Something dummy for now
    scene: Vec<String>,
}


pub struct VelloCompositor {}

impl Composable for VelloCompositor {
    type Config = VelloCompositorConfig;

    fn compositor(&self, config: Self::Config) {
        vello_compositor(config.scene);
    }
}
