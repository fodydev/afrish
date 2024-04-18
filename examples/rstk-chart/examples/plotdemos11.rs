// Translation of plotdemos11.tcl

use rish::*;

fn canvas_1(root: &rish::TkTopLevel) -> TkCanvas {
    let canvas = rish::make_canvas(root);
    canvas.background("white");

    let logxy = rish::make_logx_y(&canvas,
                                  (1.0, 1000.0),
                                  (0.0, 5.0, 1.0))
        .plot();
    
    for x in &[1.0, 2.0, 5.0, 10.0, 20.0, 50.0, 100.0, 200.0, 500.0, 1000.0] {
        logxy.plot("series1", (*x, (*x).ln()));
    }
    
    logxy.title("y = log(x)", rish::Justify::Centre);

    canvas
}

fn canvas_2(root: &rish::TkTopLevel) -> TkCanvas {
    let canvas = rish::make_canvas(root);
    canvas.background("white");

    let logxlogy = rish::make_logx_logy(&canvas,
                                  (1.0, 1000.0),
                                  (1.0, 1000000.0))
        .plot();

    logxlogy.series_colour("series1", "green");
    logxlogy.series_colour("series2", "blue");
    logxlogy.series_colour("series3", "red");
    for x in &[1.0, 2.0, 5.0, 10.0, 20.0, 50.0, 100.0, 200.0, 500.0, 1000.0] {
        logxlogy.plot("series1", (*x, (*x).sqrt()));
        logxlogy.plot("series2", (*x, (*x)*(*x)));
        logxlogy.plot("series3", (*x, (*x)*(*x).sqrt()));
    }
    logxlogy.title("y = x**n, n = 1/2, 2, 3/2", rish::Justify::Centre);
    logxlogy.x_tick_lines("grey", rish::ChartDash::Lines);

    canvas
}

fn main() {
    let root = rish::start_wish().unwrap();
    root.title("plotdemos11.rs");

    canvas_1(&root).pack().layout();
    canvas_2(&root).pack().layout();

    rish::mainloop();
}
