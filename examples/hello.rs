use rstk;

fn main() {
    let root = rstk::start_wish();

    let button = rstk::make_button(&root);
    button.text("Hello from Rust/TK");
    button.grid().layout();

    rstk::mainloop();
}

