use rstk;

fn main() {
    let root = rstk::start_wish();

    root.title("Separator/Paned Window/Label Frame");

    let panes = rstk::make_vertical_paned_window(&root);
    panes.grid().column(0).row(0).sticky("nsew").layout();
    root.grid_configure_column(0, "weight", "1");
    root.grid_configure_row(0, "weight", "1");

    let frame_h = rstk::make_label_frame(&panes);
    frame_h.text("Horizontal Separator");
    let label_1 = rstk::make_label(&frame_h);
    label_1.text("Label 1");
    let label_2 = rstk::make_label(&frame_h);
    label_2.text("Label 2");

    label_1.grid().row(0).column(0).layout();
    rstk::make_horizontal_separator(&frame_h)
        .grid().row(1).column(0).sticky("we").layout();
    label_2.grid().row(2).column(0).layout();

    frame_h.grid().row(0).column(0).padx(5).pady(5).layout();
    panes.add(&frame_h);

    let frame_v = rstk::make_label_frame(&panes);
    frame_v.text("Vertical separator");
    let label_3 = rstk::make_label(&frame_v);
    label_3.text("Label 1");
    let label_4 = rstk::make_label(&frame_v);
    label_4.text("Label 2");

    label_3.grid().row(0).column(0).layout();
    rstk::make_vertical_separator(&frame_v)
        .grid().row(0).column(1).sticky("ns").layout();
    label_4.grid().row(0).column(2).layout();

    frame_v.grid().row(0).column(0).padx(5).pady(5).layout();
    panes.add(&frame_v);

    rstk::mainloop();
}
