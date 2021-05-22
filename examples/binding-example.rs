use rstk;

fn main() {
    let root = rstk::start_wish();

    let label = rstk::make_label(&root);
    label.text("Starting ...");
    label.grid().padx(10).pady(10).layout();

    {
        let label = label.clone();
        label.clone().bind("<Enter>", move |_| { label.text("Moved mouse inside"); });
    }
    {
        let label = label.clone();
        label.clone().bind("<Leave>", move |_| { label.text("Moved mouse outside"); });
    }
    {
        let label = label.clone();
        label.clone().bind("<ButtonPress-1>", move |_| { label.text("Clicked left mouse button"); });
    }
    {
        let label = label.clone();
        label.clone().bind("<3>", move |_| { label.text("Clicked right mouse button"); });
    }
    {
        let label = label.clone();
        label.clone().bind("<Double-1>", move |_| { label.text("Double clicked"); });
    }
    label.clone().bind("<B3-Motion>",
                       move |event| {
                           label.text(&format!("Right button drag to {} {}",
                                              event.x, event.y));
                       });

    rstk::mainloop();
}
