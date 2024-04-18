use rish::*;

fn main() {
    if let Ok(root) = rish::start_wish() {

        let hello = rish::make_label(&root);
        hello.text("Hello from Rust/Tk");

        hello.grid().row(0).column(0).layout();

        rish::mainloop();
    } else {
        println!("Failed to start wish program");
    }
}
