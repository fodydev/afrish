use rstk::*;

fn main() {
    let root = rstk::start_wish().unwrap();

    let colour = rstk::colour_chooser()
        .title("Select text colour")
        .initial_colour("red")
        .show();

    let colour = colour.unwrap_or(String::from("blue"));

    let label = rstk::make_label(&root);
    label.text(&format!("In colour {}", colour));
    label.foreground(&colour);

    label.grid().layout();

    rstk::mainloop();
}
