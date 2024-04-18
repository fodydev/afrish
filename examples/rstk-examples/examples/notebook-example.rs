use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();

    root.title("Notebook Example");

    let notebook = rish::make_notebook(&root);
    notebook.grid().column(0).row(0).sticky(rish::Sticky::NESW).layout();
    root.grid_configure_column(0, "weight", "1");
    root.grid_configure_row(0, "weight", "1");

    // add three panes to the notebook
    for pane in ["Red", "Green", "Blue"].iter() {
        let frame = rish::make_frame(&notebook);
        let button = rish::make_button(&frame);
        let message = format!("Pane {}", pane);
        button.text(&message);
        button.command(move || println!("Clicked on button in {}", message));
        button.grid().column(0).row(0).padx(10).pady(10).layout();

        notebook.add(&frame, &pane);
    }

    rish::mainloop();
}
