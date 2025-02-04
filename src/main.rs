use std::cell::RefCell;
use std::rc::Rc;
use gtk4::{glib, Application, ApplicationWindow, DrawingArea, ScrolledWindow};
use gtk4::cairo::Context;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, DrawingAreaExtManual, GtkWindowExt, WidgetExt};
use render_tree::RenderTree;
use crate::layering::layer::{LayerId, LayerList};
use crate::layouter::{generate_layout, LayoutElementNode, ViewportSize};

#[allow(unused)]
mod document;
#[allow(unused)]
mod render_tree;
#[allow(unused)]
mod layouter;
mod layering;

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

    let layer_list = Rc::new(RefCell::new(layer_list));
    let layer_list_clone = layer_list.clone();
    {
        app.connect_activate(move |app| {
            build_ui(app, layer_list_clone.clone());
        });
    }
    app.run();
}

fn build_ui(app: &Application, layer_list: Rc<RefCell<LayerList>>) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Renderer")
        .default_width(800)
        .default_height(600)
        .build();

    let layer_list_for_draw = layer_list.clone();


    let visible_layer_list = Rc::new(RefCell::new(vec![true; 10]));
    let visible_layer_list_clone = visible_layer_list.clone();

    let area = DrawingArea::new();
    area.set_vexpand(true);
    area.set_hexpand(true);
    area.set_draw_func(move |_area, cr, _width, _height| {
        let layer_list_for_draw = layer_list_for_draw.borrow();

        // white background
        cr.set_source_rgb(1.0, 1.0, 1.0);
        _ = cr.paint();


        fn draw_layer(cr: &Context, layer_list: &LayerList, layer_id: LayerId) {
            fn draw_node(cr: &Context, el: &LayoutElementNode) {
                // Draw margin
                let m = el.box_model.margin_box;
                cr.set_source_rgb(243.0 / 255.0, 243.0 / 255.0, 173.0 / 255.0);
                cr.rectangle(m.x, m.y, m.width, m.height);
                _ = cr.fill();

                // Draw border
                let b = el.box_model.border_box();
                cr.set_source_rgb(48.0 / 255.0, 12.0 / 255.0, 124.0 / 255.0);
                cr.rectangle(b.x, b.y, b.width, b.height);
                _ = cr.fill();

                // Draw padding (blue)
                cr.set_source_rgb(173.0 / 255.0, 173.0 / 255.0, 247.0 / 255.0);
                let p = el.box_model.padding_box();
                cr.rectangle(p.x, p.y, p.width, p.height);
                _ = cr.fill();

                // Draw content (white fill with black stroke)
                let c = el.box_model.content_box();
                cr.set_source_rgb(173.0 / 255.0, 244.0 / 255.0, 247.0 / 255.0);
                cr.rectangle(c.x, c.y, c.width, c.height);
                _ = cr.fill();

                cr.rectangle(m.x, m.y, m.width, m.height);
                cr.set_source_rgb(1.0, 0.0, 0.0);
                _ = cr.stroke();

                // // Draw its children
                // for el_id in &el.children {
                //     let el = layer_list.layout_tree.get_node_by_id(*el_id).unwrap();
                //     draw_node(cr, layer_list, el);
                // }
            }

            let binding = layer_list.layers.borrow();
            let Some(layer) = binding.get(layer_id) else {
                return;
            };

            for el_node_id in &layer.elements {
                let el = layer_list.layout_tree.get_node_by_id(*el_node_id).unwrap();
                draw_node(cr, el);
            }
        }

        for (i, visible) in visible_layer_list_clone.borrow().iter().enumerate() {
            if *visible {
                draw_layer(cr, &layer_list_for_draw, i as LayerId);
            }
        }
        // draw_layer(cr, &layer_list_for_draw, 0);
        // draw_layer(cr, &layer_list_for_draw, 1);
    });

    let scroll = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Automatic)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .child(&area)
        .build();
    window.set_child(Some(&scroll));


    let visible_layer_list_clone = visible_layer_list.clone();

    let controller = gtk4::EventControllerKey::new();
    controller.connect_key_pressed(move |_controller, keyval, _keycode, _state| {
        let mut vll = visible_layer_list_clone.borrow_mut();

        match keyval {
            key if key == gtk4::gdk::Key::_1 => { vll[0] = !vll[0]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_2 => { vll[1] = !vll[1]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_3 => { vll[2] = !vll[2]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_4 => { vll[3] = !vll[3]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_5 => { vll[4] = !vll[4]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_6 => { vll[5] = !vll[5]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_7 => { vll[6] = !vll[6]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_8 => { vll[7] = !vll[7]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_9 => { vll[8] = !vll[8]; area.queue_draw(); }
            key if key == gtk4::gdk::Key::_0 => { vll[9] = !vll[9]; area.queue_draw(); }
            _ => (),
        }

        glib::Propagation::Proceed
    });
    window.add_controller(controller);


    window.set_default_size(800, 600);
    window.show();
}