use afrish::*;

fn main() {
    let root = afrish::start_wish().unwrap();

    root.title("button-example.rs");

    let button_1 = afrish::make_button(&root);
    button_1.text("Simple text label");
    button_1.command(|| { println!("Button-1"); });

    let image = afrish::read_image("examples/tcllogo.gif");

    let button_2 = afrish::make_button(&root);
    button_2.image(&image);
    button_2.command(|| { println!("Button-2"); });

    let button_3 = afrish::make_button(&root);
    button_3.image(&image);
    button_3.text("Tcl Logo");
    button_3.command(|| { println!("Button-3"); });

    button_3.compound(afrish::Compound::Bottom);
    button_1.state(afrish::State::Disabled);

    button_1.grid().row(0).column(0).layout();
    button_2.grid().row(0).column(1).layout();
    button_3.grid().row(0).column(2).layout();

    afrish::bind("<Return>", move |_| { button_2.invoke(); });

    afrish::mainloop();
}
