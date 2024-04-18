use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();

    root.title("progressbar-example.rs");

    let bar_1 = rish::make_progressbar(&root, 
                                       rish::Orientation::Horizontal,
                                       rish::ProgressMode::Determinate);
    bar_1.length(100);
    bar_1.value(50.0);
    bar_1.grid().row(0).column(0).padx(5).pady(5).layout();

    let bar_2 = rish::make_progressbar(&root, 
                                       rish::Orientation::Vertical,
                                       rish::ProgressMode::Determinate);
    bar_2.length(200);
    bar_2.value(80.0);
    bar_2.grid().row(0).column(1).row_span(2).padx(5).pady(5).layout();

    let bar_3 = rish::make_progressbar(&root, 
                                       rish::Orientation::Horizontal,
                                       rish::ProgressMode::Indeterminate);
    bar_3.length(100);
    bar_3.grid().row(1).column(0).padx(5).pady(5).layout();

    bar_3.start(10);

    rish::mainloop();
}
