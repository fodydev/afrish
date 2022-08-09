use rstk::*;

fn main() {
    let root = rstk::start_wish().unwrap();
    root.title("colours-example.rs");

    let label_1 = rstk::make_label(&root);
    label_1.text("default colour");
    label_1.grid().row(0).layout();

    let label_2 = rstk::make_label(&root);
    label_2.text("colour by name (\"red\")");
    label_2.foreground("red");
    label_2.grid().row(1).layout();

    let label_3 = rstk::make_label(&root);
    label_3.text("colour by RGB (#03FF2C/#FFFFFF)");
    label_3.foreground("#03FF2C");
    label_3.background("#FFFFFF");
    label_3.grid().row(2).layout();

    rstk::mainloop();
}

