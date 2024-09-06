use afrish::*;

fn main() {
    let root = afrish::start_wish().unwrap();
    root.title("spinbox-example.rs");

    let days = afrish::make_spinbox_range(&root, 1.0, 31.0, 1.0);
    let months = afrish::make_spinbox_values(&root, &["January", "February",
                                           "March", "April", "May", "June",
                                           "July", "August", "September",
                                           "October", "November", "December"]);
    months.state(afrish::State::Readonly);
    let show_date = afrish::make_button(&root);
    show_date.text("Show date");

    let day_label = afrish::make_label(&root);
    day_label.text("Day:");
    day_label.grid().column(0).row(0).layout();
    days.grid().column(1).row(0).sticky(afrish::Sticky::EW).layout();

    let month_label = afrish::make_label(&root);
    month_label.text("Month:");
    month_label.grid().column(0).row(1).layout();
    months.grid().column(1).row(1).sticky(afrish::Sticky::EW).layout();

    show_date.grid().column(0).row(2).column_span(2).layout();
    show_date.command(move || {
        println!("Date {} {}", days.value_get(), months.value_get());
    });

    afrish::mainloop();
}
