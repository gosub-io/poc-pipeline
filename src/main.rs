use crate::layouter::LayoutElementId;
use std::cell::RefCell;
use std::rc::Rc;
use gtk4::{glib, Application, ApplicationWindow, DrawingArea, EventControllerMotion, ScrolledWindow};
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, DrawingAreaExtManual, GtkWindowExt, WidgetExt};
use render_tree::RenderTree;
use crate::layering::layer::{LayerList};
use crate::layouter::{generate_layout, ViewportSize};
use crate::paint::paint_cairo;

#[allow(unused)]
mod document;
#[allow(unused)]
mod render_tree;
#[allow(unused)]
mod layouter;
mod layering;
mod paint;

/// Things that can change in the browser is stored in this structure. It keeps the current rendering pipeline (in the form of a layer_list),
/// and some things that we can control, or is controlled by the user (like current_hovered_element).
struct BrowserState {
    /// List of layers that will be visible are set to true
    visible_layer_list: Vec<bool>,
    /// If true, wireframes are drawn, otherwise complete elements are drawn
    wireframed: bool,
    /// Just show the hovered debug node in wireframe
    debug_hover: bool,
    /// When set, this is the element that is currently hovered upon
    current_hovered_element: Option<LayoutElementId>,
    /// LayerList
    layer_list: LayerList,
}

fn main() {
    // --------------------------------------------------------------------
    // Generate a DOM tree
    let doc = document::create_document();
    let mut output = String::new();
    doc.print_tree(&mut output).unwrap();
    println!("{}", output);

    // --------------------------------------------------------------------
    // Convert the DOM tree into a render-tree that has all the non-visible elements removed
    let mut render_tree = RenderTree::new(doc);
    render_tree.parse();
    render_tree.print();

    let doc_element_count = render_tree.doc.count_elements();
    let render_tree_element_count = render_tree.count_elements();

    println!("{:.2}% of the dom elements removed", (1.0 - (render_tree_element_count as f64 / doc_element_count as f64)) * 100.0);

    // --------------------------------------------------------------------
    // Layout the render-tree into a layout-tree
    let mut layout_tree = generate_layout(render_tree, ViewportSize { width: 800.0, height: 600.0 });
    layout_tree.taffy.tree.print_tree(layout_tree.taffy.root_id);

    // --------------------------------------------------------------------
    // Generate render layers
    let layer_list = LayerList::new(layout_tree);

    // --------------------------------------------------------------------
    // Render the layout-tree into a GTK window
    let app = Application::builder()
        .application_id("io.gosub.renderer")
        .build();

    let browser_state = BrowserState {
        visible_layer_list: vec![true; 10],
        wireframed: false,
        debug_hover: false,
        current_hovered_element: None,
        layer_list,
    };
    let browser_state = Rc::new(RefCell::new(browser_state));

    app.connect_activate(move |app| {
        build_ui(app, browser_state.clone());
    });

    app.run();
}

fn build_ui(app: &Application, browser_state: Rc<RefCell<BrowserState>>) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Renderer")
        .default_width(800)
        .default_height(600)
        .build();

    let area = DrawingArea::new();
    area.set_vexpand(true);
    area.set_hexpand(true);
    let browser_state_clone = browser_state.clone();
    area.set_draw_func(move |_area, cr, _width, _height| {
        let browser_state_clone = browser_state_clone.clone();
        paint_cairo(
            &browser_state_clone.borrow().layer_list,
            cr,
            browser_state_clone.borrow().visible_layer_list.clone(),
            browser_state_clone.borrow().wireframed,
            if browser_state_clone.borrow().debug_hover {
                browser_state_clone.borrow().current_hovered_element
            } else {
                None
            }
        );
    });

    let motion_controller = EventControllerMotion::new();
    let browser_state_clone = browser_state.clone();
    let area_clone = area.clone();
    motion_controller.connect_motion(move |_, x, y| {
        let mut bsm = browser_state_clone.borrow_mut();
        match bsm.layer_list.find_element_at(x, y) {
            Some(node_id) => {
                match bsm.current_hovered_element {
                    Some(current_id) => {
                        if current_id != node_id {
                            println!("OnElementLeave({})", current_id);
                            println!("OnElementEnter({})", node_id);
                            bsm.current_hovered_element = Some(node_id);
                            area_clone.queue_draw();
                        }
                    }
                    None => {
                        println!("OnElementEnter({})", node_id);
                        bsm.current_hovered_element = Some(node_id);
                        area_clone.queue_draw();
                    }
                }
            }
            None => {
                match bsm.current_hovered_element {
                    Some(current_id) => {
                        println!("OnElementLeave({})", current_id);
                        bsm.current_hovered_element = None;
                        area_clone.queue_draw();
                    }
                    None => {}
                }
            }
        }
    });
    area.add_controller(motion_controller);

    let scroll = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Automatic)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .child(&area)
        .build();
    window.set_child(Some(&scroll));



    let controller = gtk4::EventControllerKey::new();
    let browser_state_clone = browser_state.clone();
    controller.connect_key_pressed(move |_controller, keyval, _keycode, _state| {
        let mut bsm = browser_state_clone.borrow_mut();

        match keyval {
            key if key == gtk4::gdk::Key::_1 => { bsm.visible_layer_list[0] = !bsm.visible_layer_list[0]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_2 => { bsm.visible_layer_list[1] = !bsm.visible_layer_list[1]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_3 => { bsm.visible_layer_list[2] = !bsm.visible_layer_list[2]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_4 => { bsm.visible_layer_list[3] = !bsm.visible_layer_list[3]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_5 => { bsm.visible_layer_list[4] = !bsm.visible_layer_list[4]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_6 => { bsm.visible_layer_list[5] = !bsm.visible_layer_list[5]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_7 => { bsm.visible_layer_list[6] = !bsm.visible_layer_list[6]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_8 => { bsm.visible_layer_list[7] = !bsm.visible_layer_list[7]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_9 => { bsm.visible_layer_list[8] = !bsm.visible_layer_list[8]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_0 => { bsm.visible_layer_list[9] = !bsm.visible_layer_list[9]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::w => { bsm.wireframed = !bsm.wireframed; area.queue_draw(); }
            key if key == gtk4::gdk::Key::d => { bsm.debug_hover = !bsm.debug_hover; area.queue_draw(); }
            _ => (),
        }

        glib::Propagation::Proceed
    });
    window.add_controller(controller);


    window.set_default_size(800, 600);
    window.show();
}