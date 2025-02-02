use std::cell::RefCell;
use std::rc::Rc;
use gtk4::{Application, ApplicationWindow, DrawingArea, ScrolledWindow};
use gtk4::cairo::Context;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, DrawingAreaExtManual, GtkWindowExt, WidgetExt};
use taffy::{NodeId as TaffyNodeId, Rect};
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
    let mut layout_tree = generate_layout(render_tree, ViewportSize { width: 800.0, height: 600.0 });
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
    area.set_draw_func(move |_area, cr, width, height| {
        let layout_tree = layout_tree_for_draw.borrow();

        // white background
        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.paint();

        fn draw_node(cr: &Context, taffy: &taffy::TaffyTree<()>, taffy_node_id: TaffyNodeId) {
            let layout_node = taffy.layout(taffy_node_id).unwrap();

            let content = layout_node.content_box_size();
            let padding = layout_node.padding;
            let border = layout_node.border;
            let margin = layout_node.margin;

            let margin_rect = Rect::new(
                layout_node.location.x - margin.left,
                layout_node.location.y - margin.top,
                content.width + margin.left + margin.right,
                content.height + margin.top + margin.bottom,
            );

            let border_rect = Rect::new(
                layout_node.location.x - margin.left - border.left,
                layout_node.location.y - margin.top - border.top,
                content.width + margin.left + margin.right + border.left + border.right,
                content.height + margin.top + margin.bottom + border.top + border.bottom,
            );

            let padding_rect = Rect::new(
                layout_node.location.x - margin.left - border.left - padding.left,
                layout_node.location.y - margin.top - border.top - padding.top,
                content.width + margin.left + margin.right + border.left + border.right + padding.left + padding.right,
                content.height + margin.top + margin.bottom + border.top + border.bottom + padding.top + padding.bottom,
            );

            let content_rect = Rect::new(
                layout_node.location.x - margin.left - border.left - padding.left,
                layout_node.location.y - margin.top - border.top - padding.top,
                content.width + padding.left + padding.right,
                content.height + padding.top + padding.bottom,
            );

            // dbg!(content_rect);
            // dbg!(padding_rect);
            // dbg!(border_rect);
            // dbg!(margin_rect);

            // // Margin
            // cr.set_source_rgb(0.0, 1.0, 1.0);
            // cr.rectangle(
            //     margin_rect.x as f64,
            //     margin_rect.y as f64,
            //     margin_rect.width as f64,
            //     margin_rect.height as f64,
            // );
            // cr.fill();
            //
            // // Border
            // cr.set_source_rgb(0.0, 0.0, 0.0);
            // cr.rectangle(
            //     border_rect.x as f64,
            //     border_rect.y as f64,
            //     border_rect.width as f64,
            //     border_rect.height as f64,
            // );

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
