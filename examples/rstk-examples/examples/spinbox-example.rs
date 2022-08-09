use rstk::*;

fn main() {
    let root = rstk::start_wish().unwrap();
    root.title("spinbox-example.rs");

    let days = rstk::make_spinbox_range(&root, 1.0, 31.0, 1.0);
    let months = rstk::make_spinbox_values(&root, &["January", "February",
                                           "March", "April", "May", "June",
                                           "July", "August", "September",
                                           "October", "November", "December"]);
    months.state(rstk::State::Readonly);
    let show_date = rstk::make_button(&root);
    show_date.text("Show date");

    let day_label = rstk::make_label(&root);
    day_label.text("Day:");
    day_label.grid().column(0).row(0).layout();
    days.grid().column(1).row(0).sticky(rstk::Sticky::EW).layout();

    let month_label = rstk::make_label(&root);
    month_label.text("Month:");
    month_label.grid().column(0).row(1).layout();
    months.grid().column(1).row(1).sticky(rstk::Sticky::EW).layout();

    show_date.grid().column(0).row(2).column_span(2).layout();
    show_date.command(move || {
        println!("Date {} {}", days.value_get(), months.value_get());
    });

    rstk::mainloop();
}
