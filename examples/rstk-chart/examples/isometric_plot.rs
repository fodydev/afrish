use rstk::*;

fn main() {
    let root = rstk::start_wish().unwrap();
    root.title("Isometric Plot in rstk");

    let canvas = rstk::make_canvas(&root);
    canvas.width(400);
    canvas.height(400);
    canvas.background("white");
    canvas.grid().layout();

    let iso_plot = rstk::make_isometric_plot(&canvas, 
                                             (0.0, 100.0), 
                                             (0.0, 200.0), 
                                             rstk::StepSize::NoAxes);
    iso_plot.rectangle((10.0, 10.0), (50.0, 50.0), "green");
    iso_plot.filled_rectangle((20.0, 20.0), (40.0, 40.0), "red");
    iso_plot.filled_circle((70.0, 70.0), 40.0, "yellow");
    iso_plot.circle((70.0, 70.0), 42.0, "black");

    rstk::mainloop();
}
