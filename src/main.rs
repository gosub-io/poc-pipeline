use std::cell::RefCell;
use std::rc::Rc;
use gtk4::{Application, ApplicationWindow, DrawingArea, ScrolledWindow};
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

    let area = DrawingArea::new();
    area.set_vexpand(true);
    area.set_hexpand(true);
    area.set_draw_func(move |_area, cr, _width, _height| {
        let layer_list_for_draw = layer_list_for_draw.borrow();

        // white background
        cr.set_source_rgb(1.0, 1.0, 1.0);
        _ = cr.paint();


        fn draw_layer(cr: &Context, layer_list: &LayerList, layer_id: LayerId) {
            fn draw_node(cr: &Context, layer_list: &LayerList, el: &LayoutElementNode) {
                dbg!("Drawing element {}", el.id);
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

            for el_node_id in &layer_list.layers.borrow().get(layer_id).unwrap().elements {
                let el = layer_list.layout_tree.get_node_by_id(*el_node_id).unwrap();
                draw_node(cr, layer_list, el);
            }
        }

        draw_layer(cr, &layer_list_for_draw, 0);
        // draw_layer(cr, &layer_list_for_draw, 1);
    });

    let scroll = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Automatic)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .child(&area)
        .build();
    window.set_child(Some(&scroll));

    window.set_default_size(800, 600);
    window.show();
}