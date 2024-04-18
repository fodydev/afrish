use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();

    root.title("scale-example.rs");
    
    let label = rish::make_label(&root);
    label.text("Scale value: 20");

    label.grid().row(0).layout();

    let scale = rish::make_scale(&root, rish::Orientation::Horizontal);
    scale.length(200);
    scale.from(1.0);
    scale.to(100.0);
    scale.command(move |value| {
        label.text(&format!("Scale value: {}", value));
    });

    scale.grid().row(1).sticky(rish::Sticky::EW).layout();

    scale.value(20.0);

    rish::mainloop();
}
