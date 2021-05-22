use rstk;

fn main() {
    let root = rstk::start_wish();

    root.title("combobox-example.rs");

    let cb1 = rstk::make_combobox(&root, &["red", "green", "blue"]);
    let cb2 = rstk::make_combobox(&root, &["red", "green", "blue"]);
    cb2.state(rstk::State::Readonly);

    cb1.grid().row(0).column(0).pady(10).layout();
    cb2.grid().row(1).column(0).pady(10).layout();

    {
        let cb2 = cb2.clone();
        cb2.clone().bind("<<ComboboxSelected>>", 
                 move |_| { println!("cb2 is now {}", cb2.get_value()); });
    }

    let show_values = rstk::make_button(&root);
    show_values.text("Show values...");
    show_values.command(move || { 
        println!("{} and {}", cb1.get_value(), cb2.get_value()) 
    });
    show_values.grid().row(2).column(0).layout();

    rstk::mainloop();
}
