use rstk::*;

fn main() {
    let root = rstk::start_wish().unwrap();

    root.title("Notebook Example");

    let notebook = rstk::make_notebook(&root);
    notebook.grid().column(0).row(0).sticky(rstk::Sticky::NESW).layout();
    root.grid_configure_column(0, "weight", "1");
    root.grid_configure_row(0, "weight", "1");

    // add three panes to the notebook
    for pane in ["Red", "Green", "Blue"].iter() {
        let frame = rstk::make_frame(&notebook);
        let button = rstk::make_button(&frame);
        let message = format!("Pane {}", pane);
        button.text(&message);
        button.command(move || println!("Clicked on button in {}", message));
        button.grid().column(0).row(0).padx(10).pady(10).layout();

        notebook.add(&frame, &pane);
    }

    rstk::mainloop();
}
