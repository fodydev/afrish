use rstk::*;

fn main() {
    let root = rstk::start_wish().unwrap();
    root.title("Right axis example");

    let canvas = rstk::make_canvas(&root);
    canvas.width(400);
    canvas.height(400);
    canvas.background("white");
    canvas.grid().layout();

    let xy_plot = rstk::make_x_y(&canvas, (0.0, 10.0, 1.0), (0.0, 100.0, 10.0))
        .plot();
    let right_axis = rstk::make_right_axis(&canvas, (0.0, 3.0, 0.5));

    xy_plot.v_title("y = x*x");
    right_axis.v_title("y = sqrt(x)");

    xy_plot.series_colour("squares", "blue");
    right_axis.series_colour("roots", "green");
    right_axis.series_drawing_mode("roots", rstk::DrawingMode::Both);
    right_axis.series_symbol("roots", rstk::Symbol::UpFilled, 5);

    for i in 0..10 {
        let i = i as f64;
        xy_plot.plot("squares", (i, i*i));
        right_axis.plot("roots", (i, i.sqrt()));
    }

    rstk::mainloop();
}
