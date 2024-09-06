use afrish::*;

fn main() {
    let root = afrish::start_wish().unwrap();
    root.title("treeview-example.rs");

    let treeview = afrish::make_treeview(&root);
    treeview.columns(&["size", "modified"]);
    treeview.heading_text("#0", "Name");
    treeview.heading_text("size", "Size (kB)");
    treeview.heading_text("modified", "Last Modified");

    let hscroll = afrish::make_horizontal_scrollbar(&root, &treeview);
    let vscroll = afrish::make_vertical_scrollbar(&root, &treeview);

    treeview.grid().column(0).row(0).sticky(afrish::Sticky::NESW).layout();
    hscroll.grid().column(0).row(1).sticky(afrish::Sticky::EW).layout();
    vscroll.grid().column(1).row(0).sticky(afrish::Sticky::NS).layout();
    root.grid_configure_column(0, "weight", "1");
    root.grid_configure_row(0, "weight", "1");

    // -- alter alignments
    treeview.column_anchor("size", afrish::Anchor::Centre);
    treeview.column_anchor("modified", afrish::Anchor::Centre);

    // -- add a feedback label
    let label = afrish::make_label(&root);
    label.grid().column(0).row(2).sticky(afrish::Sticky::EW).layout();

    // -- populate treeview with dummy items
    let item1 = treeview.insert_item();
    item1.text("eg1.rs");
    item1.values(&["150", "18:35"]);
    let item2 = treeview.insert_item();
    item2.text("eg2.rs");
    item2.values(&["400", "15:00"]);
    let item3 = treeview.insert_item_at(0);
    item3.text("archive");
    let item4 = item3.insert_item();
    item4.text("eg3.rs");
    item4.values(&["400", "15:00"]);
    let item5 = item3.insert_item();
    item5.text("eg4.rs");
    item5.values(&["200", "16:12"]);

    // -- bind a function to respond to selections
    treeview.clone().bind("<<TreeviewSelect>>", move |_| {
        let selected_items = treeview.selected_items();
        label.text(&format!("{} selected items", selected_items.len()));
    });

    afrish::mainloop();
}

