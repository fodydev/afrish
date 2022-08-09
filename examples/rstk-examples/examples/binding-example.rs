use rstk::*;

fn main() {
    let root = rstk::start_wish().unwrap();

    let label = rstk::make_label(&root);
    label.text("Starting ...");
    label.grid().padx(10).pady(10).layout();

    {
        let labelc = label.clone();
        label.bind("<Enter>", move |_| { labelc.text("Moved mouse inside"); });
    }
    {
        let labelc = label.clone();
        label.bind("<Leave>", move |_| { labelc.text("Moved mouse outside"); });
    }
    {
        let labelc = label.clone();
        label.bind("<ButtonPress-1>", move |_| { labelc.text("Clicked left mouse button"); });
    }
    {
        let labelc = label.clone();
        label.bind("<3>", move |_| { labelc.text("Clicked right mouse button"); });
    }
    {
        let labelc = label.clone();
        label.bind("<Double-1>", move |_| { labelc.text("Double clicked"); });
    }
    label.clone().bind("<B3-Motion>",
                       move |event| {
                           label.text(&format!("Right button drag to {} {}",
                                              event.x, event.y));
                       });

    rstk::mainloop();
}
