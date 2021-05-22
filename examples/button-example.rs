use rstk;

fn main() {
    let root = rstk::start_wish();

    root.title("button-example.rs");

    let button_1 = rstk::make_button(&root);
    button_1.text("Simple text label");
    button_1.command(|| { println!("Button-1"); });

    let image = rstk::read_image("examples/tcllogo.gif");

    let button_2 = rstk::make_button(&root);
    button_2.image(&image);
    button_2.command(|| { println!("Button-2"); });

    let button_3 = rstk::make_button(&root);
    button_3.image(&image);
    button_3.text("Tcl Logo");
    button_3.command(|| { println!("Button-3"); });

    button_3.compound(rstk::Compound::Bottom);
    button_1.state(rstk::State::Disabled);

    button_1.grid().row(0).column(0).layout();
    button_2.grid().row(0).column(1).layout();
    button_3.grid().row(0).column(2).layout();

    rstk::bind("<Return>", move |_| { button_2.invoke(); });

    rstk::mainloop();
}
