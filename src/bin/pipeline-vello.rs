#[cfg(not(feature="backend_vello"))]
compile_error!("This binary can only be used with the feature 'backend_vello' enabled");

use std::num::NonZeroUsize;
use std::sync::Arc;
use vello::peniko::color;
use vello::util::{DeviceHandle, RenderContext, RenderSurface};
use vello::{wgpu, AaConfig, RenderParams, Renderer, RendererOptions};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use std::sync::RwLock;
use poc_pipeline::common;
use poc_pipeline::rendertree_builder::RenderTree;
use poc_pipeline::common::browser_state::{get_browser_state, init_browser_state, BrowserState, WireframeState};
use poc_pipeline::common::geo::{Dimension, Rect};
use poc_pipeline::compositor::Composable;
use poc_pipeline::compositor::vello::{VelloCompositor, VelloCompositorConfig};
use poc_pipeline::layering::layer::{LayerId, LayerList};
use poc_pipeline::tiler::{TileList, TileState};
use poc_pipeline::layouter::taffy::TaffyLayouter;
use poc_pipeline::layouter::CanLayout;
use poc_pipeline::painter::Painter;
use poc_pipeline::rasterizer::Rasterable;
use poc_pipeline::rasterizer::vello::VelloRasterizer;

const TILE_DIMENSION : f64 = 256.0;
const AA_CONFIGS: [AaConfig; 3] = [AaConfig::Area, AaConfig::Msaa8, AaConfig::Msaa16];


fn main() {
    // --------------------------------------------------------------------
    // Generate a DOM tree
    // let doc = common::document::create_document();
    // let doc = common::document::parser::document_from_json("tables.json");
    let doc = common::document::parser::document_from_json("news.ycombinator.com.json");
    let mut output = String::new();
    doc.print_tree(&mut output).expect("");
    println!("{}", output);

    // --------------------------------------------------------------------
    // Convert the DOM tree into a render-tree that has all the non-visible elements removed
    let mut render_tree = RenderTree::new(doc);
    render_tree.parse();
    // render_tree.print();

    // --------------------------------------------------------------------
    // Layout the render-tree into a layout-tree
    let mut layouter = TaffyLayouter::new();
    let layout_tree = layouter.layout(render_tree, None);
    layouter.print_tree();
    println!("Layout width: {}, height: {}", layout_tree.root_dimension.width, layout_tree.root_dimension.height);

    // -------------------------------------------------------------------  -
    // Generate render layers
    let layer_list = LayerList::new(layout_tree);
    // for (layer_id, layer) in layer_list.layers.read().expect("").iter() {
    //     println!("Layer: {} (order: {})", layer_id, layer.order);
    //     for element in layer.elements.iter() {
    //         println!("  Element: {}", element);
    //     }
    // }

    // --------------------------------------------------------------------
    // Tiling phase
    let mut tile_list = TileList::new(layer_list, Dimension::new(TILE_DIMENSION, TILE_DIMENSION));
    tile_list.generate();
    // tile_list.print_list();

    // --------------------------------------------------------------------
    // At this point, we have done everything we can before painting. The rest
    // is completed in the draw function of the UI.


    let browser_state = BrowserState {
        visible_layer_list: vec![true; 10],
        wireframed: WireframeState::None,
        debug_hover: false,
        current_hovered_element: None,
        tile_list: RwLock::new(tile_list),
        show_tilegrid: true,
        viewport: Rect::new(0.0, 0.0, 800.0, 600.0),
    };
    init_browser_state(browser_state);


    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();
    let _ = event_loop.run_app(&mut app);
}

struct App<'s> {
    render_ctx: RenderContext,
    renderer: Option<Renderer>,
    surface: Option<RenderSurface<'s>>, // Surface must be before window for safety during cleanup
    window: Option<Arc<Window>>,
}

impl App<'_> {
    fn new() -> Self {
        App {
            window: None,
            render_ctx: RenderContext::new(),
            renderer: None,
            surface: None,
        }
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let mut attribs = Window::default_attributes();
        attribs.title = "Vello Pipeline Test".to_string();
        let window = Arc::new(event_loop.create_window(attribs).unwrap());

        let size = window.inner_size();
        let surface_future =
            self.render_ctx
                .create_surface(window.clone(), size.width, size.height, wgpu::PresentMode::AutoVsync);
        let surface = pollster::block_on(surface_future).expect("Failed to create surface");

        let dev_handle = &self.render_ctx.devices[surface.dev_id];

        let renderer = Renderer::new(
            &dev_handle.device,
            RendererOptions {
                surface_format: Some(surface.format),
                use_cpu: false,
                antialiasing_support: AA_CONFIGS.iter().copied().collect(),
                num_init_threads: NonZeroUsize::new(0),
            },
        );

        self.window = Some(window);
        self.surface = Some(surface);
        self.renderer = Some(renderer.unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                self.render_ctx
                    .resize_surface(self.surface.as_mut().unwrap(), size.width, size.height);
            }
            WindowEvent::RedrawRequested => {
                let surface = self.surface.as_ref().unwrap();

                let dev_id = surface.dev_id;
                let DeviceHandle { device, queue, .. } = &self.render_ctx.devices[dev_id];

                let width = surface.config.width;
                let height = surface.config.height;


                let binding = get_browser_state();
                let state = binding.read().unwrap();
                let vis_layers = state.visible_layer_list.clone();
                drop(state);

                if vis_layers[0] {
                    do_paint(LayerId::new(0));
                    do_rasterize(device, queue, LayerId::new(0));
                }
                if vis_layers[1] {
                    do_paint(LayerId::new(1));
                    do_rasterize(device, queue, LayerId::new(1));
                }

                let surface_texture = surface
                    .surface
                    .get_current_texture()
                    .expect("Failed to get current texture");

                let render_params = RenderParams {
                    base_color: color::palette::css::LIGHT_BLUE,
                    width,
                    height,
                    antialiasing_method: AaConfig::Area,
                };

                let scene = VelloCompositor::compose(VelloCompositorConfig{});

                let _ = self.renderer.as_mut().unwrap().render_to_surface(
                    device,
                    queue,
                    &scene,
                    &surface_texture,
                    &render_params,
                );

                surface_texture.present();
            }
            _ => (),
        }
    }
}

fn do_paint(layer_id: LayerId) {
    let binding = get_browser_state();
    let state = binding.read().unwrap();

    let painter = Painter::new(state.tile_list.read().unwrap().layer_list.clone());

    let tile_ids = state.tile_list.read().unwrap().get_intersecting_tiles(layer_id, state.viewport);
    for tile_id in tile_ids {
        // get tile
        let mut binding = state.tile_list.write().expect("Failed to get tile list");
        let Some(tile) = binding.get_tile_mut(tile_id) else {
            log::warn!("Tile not found: {:?}", tile_id);
            continue;
        };

        // if not dirty, no need to render and continue
        if tile.state == TileState::Clean {
            continue;
        }

        // Paint all the elements in each tile
        for tiled_layout_element in &mut tile.elements {
            tiled_layout_element.paint_commands = painter.paint(tiled_layout_element);
        }
    }
}

fn do_rasterize(device: &wgpu::Device, queue: &wgpu::Queue, layer_id: LayerId) {
    let binding = get_browser_state();
    let state = binding.read().unwrap();

    let tile_ids = state.tile_list.read().unwrap().get_intersecting_tiles(layer_id, state.viewport);
    for tile_id in tile_ids {
        // get tile
        let mut binding = state.tile_list.write().expect("Failed to get tile list");
        let Some(tile) = binding.get_tile(tile_id) else {
            log::warn!("Tile not found: {:?}", tile_id);
            continue;
        };

        // if not dirty, no need to render and continue
        if tile.state == TileState::Clean {
            continue;
        }

        // Rasterize the tile into a texture
        let rasterizer = VelloRasterizer::new(device, queue);
        let texture_id = rasterizer.rasterize(tile);

        let Some(tile) = binding.get_tile_mut(tile_id) else {
            log::warn!("Tile not found: {:?}", tile_id);
            continue;
        };

        tile.texture_id = Some(texture_id);
        tile.state = TileState::Clean;
    }
}