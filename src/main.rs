use std::sync::RwLock;
use gtk4::{glib, Adjustment, Application, ApplicationWindow, DrawingArea, EventControllerMotion, ScrolledWindow};
use gtk4::glib::clone;
use gtk4::prelude::{AdjustmentExt, ApplicationExt, ApplicationExtManual, DrawingAreaExt, DrawingAreaExtManual, GtkWindowExt, WidgetExt};
use render_tree::RenderTree;
use crate::browser_state::{get_browser_state, init_browser_state, BrowserState};
use crate::geo::Rect;
use crate::layering::layer::{LayerId, LayerList};
use crate::layouter::{generate_layout, ViewportSize};
use crate::paint::paint_cairo;
use crate::painter::Painter;
use crate::tiler::{TileList, TileState};

const TILE_DIMENSION : usize = 220;

#[allow(unused)]
mod document;
#[allow(unused)]
mod render_tree;
#[allow(unused)]
mod layouter;
mod layering;
mod paint;
#[allow(unused)]
mod tiler;
#[allow(unused)]
mod geo;
mod browser_state;
#[allow(unused)]
mod painter;

fn main() {
    // --------------------------------------------------------------------
    // Generate a DOM tree
    println!("\n\n\n\n\n--[ DOM TREE ]----------------------------------");
    let doc = document::create_document();
    let mut output = String::new();
    doc.print_tree(&mut output).unwrap();
    println!("{}", output);

    // --------------------------------------------------------------------
    // Convert the DOM tree into a render-tree that has all the non-visible elements removed
    println!("\n\n\n\n\n--[ RENDER TREE ]----------------------------------");
    let mut render_tree = RenderTree::new(doc);
    render_tree.parse();
    render_tree.print();

    let doc_element_count = render_tree.doc.count_elements();
    let render_tree_element_count = render_tree.count_elements();

    println!("{:.2}% of the dom elements removed", (1.0 - (render_tree_element_count as f64 / doc_element_count as f64)) * 100.0);

    // --------------------------------------------------------------------
    // Layout the render-tree into a layout-tree
    println!("\n\n\n\n\n--[ LAYOUT TREE ]----------------------------------");
    let mut layout_tree = generate_layout(render_tree, ViewportSize { width: 800.0, height: 600.0 });
    layout_tree.taffy.tree.print_tree(layout_tree.taffy.root_id);
    println!("Layout width: {}, height: {}", layout_tree.root_width, layout_tree.root_height);

    // --------------------------------------------------------------------
    // Generate render layers
    println!("\n\n\n\n\n--[ LAYER LIST ]----------------------------------");
    let layer_list = LayerList::new(layout_tree);
    for (layer_id, layer) in layer_list.layers.read().unwrap().iter() {
        println!("Layer: {} (order: {})", layer_id, layer.order);
        for element in layer.elements.iter() {
            println!("  Element: {}", element);
        }
    }

    // --------------------------------------------------------------------
    // Tiling phase
    println!("\n\n\n\n\n--[ TILING ]----------------------------------");
    let mut tile_list = TileList::new(layer_list, TILE_DIMENSION);
    tile_list.generate();
    tile_list.print_list();

    // --------------------------------------------------------------------
    // At this point, we have done everything we can before painting. The rest
    // is completed in the draw function of the UI.


    // Render the layout-tree into a GTK window
    let app = Application::builder()
        .application_id("io.gosub.renderer")
        .build();

    let browser_state = BrowserState {
        visible_layer_list: vec![true; 10],
        wireframed: false,
        debug_hover: false,
        current_hovered_element: None,
        tile_list: RwLock::new(tile_list),
        show_tilegrid: true,
        viewport: Rect::new(0.0, 0.0, 800.0, 600.0),
        _marker: Default::default(),
    };
    init_browser_state(browser_state);

    app.connect_activate(move |app| {
        build_ui(app);
    });

    app.run();
}


fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Renderer")
        .default_width(800)
        .default_height(600)
        .build();

    let area = DrawingArea::new();
    // area.set_vexpand(true);
    // area.set_hexpand(true);
    area.set_content_height(800);
    area.set_content_width(600);
    area.set_draw_func(move |_area, cr, _width, _height| {
        // --------------------------------------------------------------------
        // Next phase in the pipeline: we need to find which tiles we need to paint
        println!("\n\n\n\n\n--[ PAINTING ]----------------------------------");
        let binding = get_browser_state();
        let state = binding.read().unwrap();
        let tile_ids = state.tile_list.read().unwrap().get_intersecting_tiles(LayerId::new(0), state.viewport);
        for tile_id in tile_ids {
            print!("Rendering tile {:?}: ", tile_id);
            // get tile
            let mut binding = state.tile_list.write().unwrap();
            let Some(tile) = binding.get_tile_mut(tile_id) else {
                log::warn!("Tile not found: {:?}", tile_id);
                println!("tile not found?");
                continue;
            };

            // if not dirty, no need to render and continue
            if tile.state == TileState::Clean {
                println!("tile is already rendered. Not rendering again.");
                continue;
            }

            let painter = Painter::new();
            let texture = painter.paint(tile);
            tile.texture = Some(texture);

            // Render the given tile
            println!("Rendering tile");
            tile.state = TileState::Clean;
        }


        let binding = get_browser_state();
        let state = binding.read().unwrap();
        // Paint the layer list to the cairo context. Also pass a few flags that allows
        // us to control what is exactly being rendered.
        paint_cairo(
            &state.tile_list.read().unwrap(),
            cr,
            // List of the layers to render
            state.visible_layer_list.clone(),
            // When true, only render the wireframes of the layout elements
            state.wireframed,
            // When set, only render the hovered element
            if state.debug_hover {
                state.current_hovered_element
            } else {
                None
            },
            state.show_tilegrid,
        );
    });

    // When we move the mouse, we can detect which element is currently hovered upon
    // This allows us to trigger events (OnElementLeave, onElementEnter). At that point,
    // we trigger a redraw, since there can be things that need to be updated.
    let motion_controller = EventControllerMotion::new();
    let area_clone = area.clone();
    motion_controller.connect_motion(move |_, x, y| {
        let binding = get_browser_state();
        let mut state = binding.write().unwrap();

        let el = state.tile_list.read().unwrap().layer_list.find_element_at(x, y).clone();
        match (state.current_hovered_element, el) {
            (Some(current_id), Some(new_id)) if current_id != new_id => {
                println!("OnElementLeave({})", current_id);
                println!("OnElementEnter({})", new_id);
                state.current_hovered_element = Some(new_id);
                area_clone.queue_draw();
            }
            (None, Some(new_id)) => {
                println!("OnElementEnter({})", new_id);
                state.current_hovered_element = Some(new_id);
                area_clone.queue_draw();
            }
            (Some(current_id), None) => {
                println!("OnElementLeave({})", current_id);
                state.current_hovered_element = None;
                area_clone.queue_draw();
            }
            _ => {},
        }
    });
    area.add_controller(motion_controller);

    let scroll = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Always)
        .vscrollbar_policy(gtk4::PolicyType::Always)
        .child(&area)
        .build();
    window.set_child(Some(&scroll));

    connect_viewport_signals(&scroll, &area);

    // Add keyboard shortcuts to trigger some of the rendering options
    let controller = gtk4::EventControllerKey::new();
    controller.connect_key_pressed(move |_controller, keyval, _keycode, _state| {
        let binding = get_browser_state();
        let mut state = binding.write().unwrap();

        match keyval {
            // numeric keys triggers the visibility of the layers
            key if key == gtk4::gdk::Key::_1 => { state.visible_layer_list[0] = !state.visible_layer_list[0]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_2 => { state.visible_layer_list[1] = !state.visible_layer_list[1]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_3 => { state.visible_layer_list[2] = !state.visible_layer_list[2]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_4 => { state.visible_layer_list[3] = !state.visible_layer_list[3]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_5 => { state.visible_layer_list[4] = !state.visible_layer_list[4]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_6 => { state.visible_layer_list[5] = !state.visible_layer_list[5]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_7 => { state.visible_layer_list[6] = !state.visible_layer_list[6]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_8 => { state.visible_layer_list[7] = !state.visible_layer_list[7]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_9 => { state.visible_layer_list[8] = !state.visible_layer_list[8]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_0 => { state.visible_layer_list[9] = !state.visible_layer_list[9]; area.queue_draw(); }
            // toggle wireframed elements
            key if key == gtk4::gdk::Key::w => { state.wireframed = !state.wireframed; area.queue_draw(); }
            // toggle displaying only the hovered element
            key if key == gtk4::gdk::Key::d => { state.debug_hover = !state.debug_hover; area.queue_draw(); }
            // toggle tile grid
            key if key == gtk4::gdk::Key::t => { state.show_tilegrid = !state.show_tilegrid; area.queue_draw(); }
            _ => (),
        }

        glib::Propagation::Proceed
    });
    window.add_controller(controller);

    window.set_default_size(1024, 768);
    window.show();
}

// Function to set up viewport event listeners
fn connect_viewport_signals(scroll: &ScrolledWindow, area: &DrawingArea) {
    let hadjustment = scroll.hadjustment();
    let vadjustment = scroll.vadjustment();

    // Connect to the scroll changes
    hadjustment.connect_value_changed(clone!(
        #[weak] area,
        #[weak] vadjustment,
        move |adj| {
            on_viewport_changed(&area, adj, &vadjustment);
        }
    ));

    vadjustment.connect_value_changed(clone!(
        #[weak] area,
        #[weak] hadjustment,
        move |adj| {
            on_viewport_changed(&area, &hadjustment, adj);
        }
    ));

    // Connect to the DrawingArea's resize signal
    area.connect_resize(clone!(
        #[weak] area,
        #[weak] hadjustment,
        #[weak] vadjustment,
        move |_, _, _| {
            on_viewport_changed(&area, &hadjustment, &vadjustment);
        }
    ));
}

// Function to handle viewport changes
fn on_viewport_changed(area: &DrawingArea, hadj: &Adjustment, vadj: &Adjustment) {
    let x = hadj.value();       // Scroll offset X
    let y = vadj.value();       // Scroll offset Y
    let width = hadj.page_size(); // Visible width
    let height = vadj.page_size(); // Visible height

    println!("Visible viewport: x={} y={} width={} height={}", x, y, width, height);

    let binding = get_browser_state();
    let mut state = binding.write().unwrap();
    state.viewport = Rect::new(x, y, width, height);

    area.queue_draw(); // Request re-draw if necessary
}
