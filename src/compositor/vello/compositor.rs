use gtk4::cairo;
use gtk4::cairo::ImageSurface;
use crate::browser_state::get_browser_state;
use crate::layering::layer::LayerId;
use crate::rasterize::texture_store::get_texture_store;

pub fn vello_compositor(scene: Vec<String>) {
    println!("This is a dummy vello compositor. You probably will see nothing on the screen now");
}