use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();

    let colour = rish::colour_chooser()
        .title("Select text colour")
        .initial_colour("red")
        .show();

    let colour = colour.unwrap_or(String::from("blue"));

    let label = rish::make_label(&root);
    label.text(&format!("In colour {}", colour));
    label.foreground(&colour);

    label.grid().layout();

    rish::mainloop();
}
