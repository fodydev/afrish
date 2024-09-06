use afrish::*;

fn setup(root: &impl afrish::TkWidget) {
    let hello = afrish::make_label(root);
    hello.text("Hello from Rust/Tk");

    hello.grid().layout();
}

fn main() {
    let root = afrish::start_wish().unwrap();

    setup(&root);

    afrish::mainloop();
}
