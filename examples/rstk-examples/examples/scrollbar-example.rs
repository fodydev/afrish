use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();

    root.title("scrollbar-example.rs");
    let listbox = rish::make_listbox(&root, &[]);
    listbox.height(5);

    for i in 1..=100 {
        listbox.append(&format!("Line {} of 100", i));
    }

    let scrollbar = rish::make_vertical_scrollbar(&root, &listbox);
    let status = rish::make_label(&root);
    status.text("Status message here");

    listbox.grid().column(0).row(0).sticky(rish::Sticky::NESW).layout();
    scrollbar.grid().column(1).row(0).sticky(rish::Sticky::NS).layout();
    status.grid().column(0).row(1).column_span(2)
        .sticky(rish::Sticky::EW).layout();

    root.grid_configure_column(0, "weight", "1");
    root.grid_configure_row(0, "weight", "1");

    rish::mainloop();
}
