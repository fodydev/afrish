use afrish;

fn main() {
    afrish::start_wish().unwrap();

    let file = afrish::open_file_chooser()
        .title("Open")
        .file_types(&[("C++", ".cpp"), ("Rust", ".rs"), ("Any", "*")])
        .show();

    println!("Filename: {}", file.unwrap_or(String::from("cancelled")));
    
    let file = afrish::save_file_chooser()
        .title("Save")
        .file_types(&[("C++", ".cpp"), ("Rust", ".rs"), ("Any", "*")])
        .show();

    println!("Filename: {}", file.unwrap_or(String::from("cancelled")));

    afrish::mainloop();
}
