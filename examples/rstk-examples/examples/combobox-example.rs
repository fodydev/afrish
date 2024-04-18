use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();

    root.title("combobox-example.rs");

    let cb1 = rish::make_combobox(&root, &["red", "green", "blue"]);
    let cb2 = rish::make_combobox(&root, &["red", "green", "blue"]);
    cb2.state(rish::State::Readonly);

    cb1.grid().row(0).column(0).pady(10).layout();
    cb2.grid().row(1).column(0).pady(10).layout();

    {
        let cb2c = cb2.clone();
        cb2.bind("<<ComboboxSelected>>", 
                 move |_| { println!("cb2 is now {}", cb2c.value_get()); });
    }

    let show_values = rish::make_button(&root);
    show_values.text("Show values...");
    show_values.command(move || { 
        println!("{} and {}", cb1.value_get(), cb2.value_get()) 
    });
    show_values.grid().row(2).column(0).layout();

    rish::mainloop();
}
