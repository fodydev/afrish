use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();

    root.title("grid-example.rs");

    // first, make some widgets and parent frames
    let content = rish::make_frame(&root);
    let frame = rish::make_frame(&content);
    frame.border_width(5);
    frame.relief(rish::Relief::Ridge);
    frame.width(200);
    frame.height(100);

    let name_label = rish::make_label(&content);
    name_label.text("Name");

    let name = rish::make_entry(&content);
    let cb_1 = rish::make_check_button(&content);
    cb_1.text("One");
    let cb_2 = rish::make_check_button(&content);
    cb_2.text("Two");
    let cb_3 = rish::make_check_button(&content);
    cb_3.text("Three");
    let ok = rish::make_button(&content);
    ok.text("OK");
    let cancel = rish::make_button(&content);
    cancel.text("Cancel");

    // -- some adjustments to the widgets/frames

    content.padding(&[3, 3, 12, 12]);
    cb_1.selected(true);
    cb_2.selected(false);
    cb_3.selected(true);

    // -- layout the widgets in the grid
    content.grid().sticky(rish::Sticky::NESW).layout();
    frame.grid().column_span(3).row_span(2).sticky(rish::Sticky::NESW).layout();
    name_label.grid().row(0).column(3).column_span(2).sticky(rish::Sticky::NW).padx(5).layout();
    name.grid().row(1).column(3).column_span(2).sticky(rish::Sticky::NEW).pady(5).padx(5).layout();
    cb_1.grid().row(3).column(0).layout();
    cb_2.grid().row(3).column(1).layout();
    cb_3.grid().row(3).column(2).layout();
    ok.grid().row(3).column(3).layout();
    cancel.grid().row(3).column(4).layout();

    // -- tidy up the layout and resizing properties
    root.grid_configure_column(0, "weight", "1");
    root.grid_configure_row(0, "weight", "1");
    content.grid_configure_column(0, "weight", "3");
    content.grid_configure_column(1, "weight", "3");
    content.grid_configure_column(2, "weight", "3");
    content.grid_configure_column(3, "weight", "1");
    content.grid_configure_column(4, "weight", "1");
    content.grid_configure_row(1, "weight", "1");

    rish::mainloop();
}
