use afrish::*;

fn main() {
    let root = afrish::start_wish().unwrap();

    let button = afrish::make_button(&root);
    button.text("Hello from Rust/TK");
    button.grid().layout();

    afrish::mainloop();
}

