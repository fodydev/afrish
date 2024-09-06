use afrish::*;

fn main() {
    let root = afrish::start_wish().unwrap();

    root.title("scrollbar-example.rs");
    let listbox = afrish::make_listbox(&root, &[]);
    listbox.height(5);

    for i in 1..=100 {
        listbox.append(&format!("Line {} of 100", i));
    }

    let scrollbar = afrish::make_vertical_scrollbar(&root, &listbox);
    let status = afrish::make_label(&root);
    status.text("Status message here");

    listbox.grid().column(0).row(0).sticky(afrish::Sticky::NESW).layout();
    scrollbar.grid().column(1).row(0).sticky(afrish::Sticky::NS).layout();
    status.grid().column(0).row(1).column_span(2)
        .sticky(afrish::Sticky::EW).layout();

    root.grid_configure_column(0, "weight", "1");
    root.grid_configure_row(0, "weight", "1");

    afrish::mainloop();
}
