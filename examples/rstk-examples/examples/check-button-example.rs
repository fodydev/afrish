use rstk::*;

fn main() {
    let root = rstk::start_wish().unwrap();

    root.title("check-button-example.rs");

    let button_1 = rstk::make_check_button(&root);
    button_1.text("Simple text label");
    button_1.command(|value| { println!("Button-1 now {}", value); });

    let image = rstk::read_image("examples/tcllogo.gif");
    let button_2 = rstk::make_check_button(&root);
    button_2.image(&image);
    button_2.command(|value| { println!("Button-2 now {}", value); });

    let button_3 = rstk::make_check_button(&root);
    button_3.image(&image);
    button_3.text("Tcl Logo");
    button_3.command(|value| { println!("Button-3 now {}", value); });

    let show = rstk::make_button(&root);
    show.text("Show states");
    {
        let button_1 = button_1.clone();
        let button_2 = button_2.clone();
        let button_3 = button_3.clone();

        show.command(move || { println!("Buttons now {} {} {}", 
                                        button_1.is_selected(), 
                                        button_2.is_selected(), 
                                        button_3.is_selected() )
        });
    }
    
    button_1.selected(true);
    button_3.compound(rstk::Compound::Bottom);

    button_1.grid().row(0).column(0).padx(5).pady(5).layout();
    button_2.grid().row(0).column(1).padx(5).pady(5).layout();
    button_3.grid().row(0).column(2).padx(5).pady(5).layout();
    show.grid().row(1).column_span(3).layout();

    rstk::mainloop();
}
