use rstk::*;

fn main() {
    let root = rstk::start_wish().unwrap();

    root.title("scale-example.rs");
    
    let label = rstk::make_label(&root);
    label.text("Scale value: 20");

    label.grid().row(0).layout();

    let scale = rstk::make_scale(&root, rstk::Orientation::Horizontal);
    scale.length(200);
    scale.from(1.0);
    scale.to(100.0);
    scale.command(move |value| {
        label.text(&format!("Scale value: {}", value));
    });

    scale.grid().row(1).sticky(rstk::Sticky::EW).layout();

    scale.value(20.0);

    rstk::mainloop();
}
