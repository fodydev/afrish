use afrish::*;

fn main() {
    if let Ok(root) = afrish::start_wish() {
        let hello = afrish::make_label(&root);
        hello.text("Hello from Rust/Tk");

        hello.grid().row(0).column(0).layout();

        afrish::mainloop();
    } else {
        println!("Failed to start wish program");
    }
}
