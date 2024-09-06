use afrish::*;

fn main() {
    let root = afrish::start_wish().unwrap();

    root.title("radio-button-example.rs");

    let button_1 = afrish::make_radio_button(&root, "colours", "red");
    button_1.text("Red");
    let button_2 = afrish::make_radio_button(&root, "colours", "green");
    button_2.text("Green");
    let button_3 = afrish::make_radio_button(&root, "colours", "blue");
    button_3.text("Blue");

    button_1.grid().column(0).row(0).padx(5).pady(5).layout();
    button_2.grid().column(1).row(0).padx(5).pady(5).layout();
    button_3.grid().column(2).row(0).padx(5).pady(5).layout();
    button_3.value("green");

    let show_button = afrish::make_button(&root);
    show_button.text("Show state");
    show_button.command(move || {
        println!("Colour: {}", button_3.value_get());
    });

    show_button.grid().column(0).row(1).column_span(3).layout();

    afrish::mainloop();
}
