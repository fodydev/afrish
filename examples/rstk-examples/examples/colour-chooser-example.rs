use afrish::*;

fn main() {
    let root = afrish::start_wish().unwrap();

    let colour = afrish::colour_chooser()
        .title("Select text colour")
        .initial_colour("red")
        .show();

    let colour = colour.unwrap_or(String::from("blue"));

    let label = afrish::make_label(&root);
    label.text(&format!("In colour {}", colour));
    label.foreground(&colour);

    label.grid().layout();

    afrish::mainloop();
}
