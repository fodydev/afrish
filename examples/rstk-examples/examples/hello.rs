use rstk::*;

fn main() {
    let root = rstk::start_wish().unwrap();

    let button = rstk::make_button(&root);
    button.text("Hello from Rust/TK");
    button.grid().layout();

    rstk::mainloop();
}

