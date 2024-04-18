use rish;

fn main() {
    rish::start_wish().unwrap();

    let file = rish::open_file_chooser()
        .title("Open")
        .file_types(&[("C++", ".cpp"), ("Rust", ".rs"), ("Any", "*")])
        .show();

    println!("Filename: {}", file.unwrap_or(String::from("cancelled")));
    
    let file = rish::save_file_chooser()
        .title("Save")
        .file_types(&[("C++", ".cpp"), ("Rust", ".rs"), ("Any", "*")])
        .show();

    println!("Filename: {}", file.unwrap_or(String::from("cancelled")));

    rish::mainloop();
}
