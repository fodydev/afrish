use rstk::*;

fn main() {
    let root = rstk::start_wish().unwrap();
    root.title("Polar Plot in rstk");

    let canvas = rstk::make_canvas(&root);
    canvas.width(400);
    canvas.height(300);
    canvas.background("white");
    canvas.grid().layout();

    let polar_plot = rstk::make_polar(&canvas, (3.0, 1.0));

    polar_plot.series_colour("line", "blue");
    polar_plot.series_drawing_mode("line", rstk::DrawingMode::Both);
    polar_plot.series_symbol("line", rstk::Symbol::Cross, 5);

    for i in 0..30 {
        let i = i as f64;
        let r = i/10.0;
        let a = i*24.0;

        polar_plot.plot("line", (r, a));
    }

    polar_plot.draw_labelled_dot((2.0, 60.0), "Mark", rstk::Location::North);

    rstk::mainloop();
}

