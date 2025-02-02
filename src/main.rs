use std::cell::RefCell;
use std::rc::Rc;
use gtk4::{Application, ApplicationWindow, DrawingArea, ScrolledWindow};
use gtk4::cairo::Context;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, DrawingAreaExtManual, GtkWindowExt, WidgetExt};
use taffy::NodeId as TaffyNodeId;
use render_tree::RenderTree;
use crate::layouter::{generate_layout, LayoutTree, ViewportSize};

#[allow(unused)]
mod document;
#[allow(unused)]
mod render_tree;
#[allow(unused)]
mod layouter;

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
    let mut layout_tree = generate_layout(render_tree, ViewportSize { width: 400.0, height: 300.0 });
    layout_tree.taffy_tree.print_tree(layout_tree.taffy_root_id);

    let layout_tree = Rc::new(RefCell::new(layout_tree));

    // --------------------------------------------------------------------
    // Render the layout-tree into a GTK window
    let app = Application::builder()
        .application_id("io.gosub.renderer")
        .build();
    let layout_tree_clone = layout_tree.clone();
    {
        app.connect_activate(move |app| {
            build_ui(app, layout_tree_clone.clone());
        });
    }
    app.run();
}

fn build_ui(app: &Application, layout_tree: Rc<RefCell<LayoutTree>>) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Renderer")
        .default_width(800)
        .default_height(600)
        .build();

    let layout_tree_for_draw = layout_tree.clone();

    let area = DrawingArea::new();
    area.set_vexpand(true);
    area.set_hexpand(true);
    area.set_draw_func(move |_area, cr, _width, _height| {
        let layout_tree = layout_tree_for_draw.borrow();

        // white background
        cr.set_source_rgb(1.0, 1.0, 1.0);
        _ = cr.paint();

        fn draw_node(cr: &Context, taffy: &taffy::TaffyTree<()>, taffy_node_id: TaffyNodeId) {
            let layout_node = taffy.layout(taffy_node_id).unwrap();

            let bm = layouter::taffy::convert_to_boxmodel(&layout_node);
            // dbg!(&bm);
            // dbg!(&bm.margin_box);
            // dbg!(&bm.border_box());
            // dbg!(&bm.padding_box());
            // dbg!(&bm.content_box());

            // Draw margin
            let m = bm.margin_box;
            cr.set_source_rgb(243.0 / 255.0, 243.0 / 255.0, 173.0 / 255.0);
            cr.rectangle(m.x, m.y, m.width, m.height);
            _ = cr.fill();

            // Draw border
            let b = bm.border_box();
            cr.set_source_rgb(48.0 / 255.0, 12.0 / 255.0, 124.0 / 255.0);
            cr.rectangle(b.x, b.y, b.width, b.height);
            _ = cr.fill();

            // Draw padding (blue)
            cr.set_source_rgb(173.0 / 255.0, 173.0 / 255.0, 247.0 / 255.0);
            let p = bm.padding_box();
            cr.rectangle(p.x, p.y, p.width, p.height);
            _ = cr.fill();

            // Draw content (white fill with black stroke)
            let c = bm.content_box();
            cr.set_source_rgb(173.0 / 255.0, 244.0 / 255.0, 247.0 / 255.0);
            cr.rectangle(c.x, c.y, c.width, c.height);
            _ = cr.fill();

            for child_id in taffy.children(taffy_node_id).unwrap() {
                draw_node(cr, taffy, child_id);
            }
        }

        draw_node(cr, &layout_tree.taffy_tree, layout_tree.taffy_root_id);
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
