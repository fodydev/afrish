use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();

    root.title("Separator/Paned Window/Label Frame");

    let panes = rish::make_paned_window(&root, rish::Orientation::Vertical);
    panes.grid().column(0).row(0).sticky(rish::Sticky::NESW).layout();
    root.grid_configure_column(0, "weight", "1");
    root.grid_configure_row(0, "weight", "1");

    let frame_h = rish::make_label_frame(&panes);
    frame_h.text("Horizontal Separator");
    let label_1 = rish::make_label(&frame_h);
    label_1.text("Label 1");
    let label_2 = rish::make_label(&frame_h);
    label_2.text("Label 2");

    label_1.grid().row(0).column(0).layout();
    rish::make_separator(&frame_h, rish::Orientation::Horizontal)
        .grid().row(1).column(0).sticky(rish::Sticky::EW).layout();
    label_2.grid().row(2).column(0).layout();

    frame_h.grid().row(0).column(0).padx(5).pady(5).layout();
    panes.add(&frame_h);

    let frame_v = rish::make_label_frame(&panes);
    frame_v.text("Vertical separator");
    let label_3 = rish::make_label(&frame_v);
    label_3.text("Label 1");
    let label_4 = rish::make_label(&frame_v);
    label_4.text("Label 2");

    label_3.grid().row(0).column(0).layout();
    rish::make_separator(&frame_v, rish::Orientation::Vertical)
        .grid().row(0).column(1).sticky(rish::Sticky::NS).layout();
    label_4.grid().row(0).column(2).layout();

    frame_v.grid().row(0).column(0).padx(5).pady(5).layout();
    panes.add(&frame_v);

    rish::mainloop();
}
