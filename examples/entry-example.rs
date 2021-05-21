use rstk;

fn main() {
    let root = rstk::start_wish();

    root.title("entry-example.rs");

    let entry_1 = rstk::make_entry(&root);
    let entry_2 = rstk::make_entry(&root);
    entry_2.show('*');

    let name_label = rstk::make_label(&root);
    name_label.text("Name:");
    name_label.grid().row(0).column(0).layout();
    entry_1.grid().row(0).column(1).pady(5).layout();

    let password_label = rstk::make_label(&root);
    password_label.text("Password:");
    password_label.grid().row(1).column(0).layout();
    entry_2.grid().row(1).column(1).pady(5).layout();

    let show_button = rstk::make_button(&root);
    show_button.text("Show entries");
    show_button.command(move || {
        println!("{} - {}", entry_1.value(), entry_2.value());
    });
    show_button.grid().row(2).column(0).column_span(2).layout();

    rstk::mainloop();
}
