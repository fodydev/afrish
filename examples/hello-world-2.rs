use rish::*;

fn setup(root: &impl rish::TkWidget) {
    let hello = rish::make_label(root);
    hello.text("Hello from Rust/Tk");

    hello.grid().layout();
}

fn main() {
    let root = rish::start_wish().unwrap();

    setup(&root);

    rish::mainloop();
}
