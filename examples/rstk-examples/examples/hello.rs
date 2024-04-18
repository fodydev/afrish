use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();

    let button = rish::make_button(&root);
    button.text("Hello from Rust/TK");
    button.grid().layout();

    rish::mainloop();
}

