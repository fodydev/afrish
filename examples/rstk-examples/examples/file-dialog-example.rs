use rstk;

fn main() {
    rstk::start_wish().unwrap();

    let file = rstk::open_file_chooser()
        .title("Open")
        .file_types(&[("C++", ".cpp"), ("Rust", ".rs"), ("Any", "*")])
        .show();

    println!("Filename: {}", file.unwrap_or(String::from("cancelled")));
    
    let file = rstk::save_file_chooser()
        .title("Save")
        .file_types(&[("C++", ".cpp"), ("Rust", ".rs"), ("Any", "*")])
        .show();

    println!("Filename: {}", file.unwrap_or(String::from("cancelled")));

    rstk::mainloop();
}
