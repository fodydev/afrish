// Translation of plotdemos19.tcl

use rish::*;

fn std_canvas(root: &rish::TkTopLevel) -> rish::TkCanvas {
    let canvas = rish::make_canvas(root);
    canvas.background("white");
    canvas.width(600);
    canvas.height(300);
    canvas
}

fn chart_1(root: &rish::TkTopLevel) -> rish::TkCanvas {
    let canvas = std_canvas(root);

    let chart = rish::make_x_y(&canvas, (-10.0, 10.0, 5.0), (-10.0, 10.0, 5.0))
        .axes_at_zero(true)
        .plot();
    chart.title("Axes at the origin (axes_at_zero true)", rish::Justify::Centre);
    chart.plot("data", (0.0, 1.0));
    chart.plot("data", (1.0, 2.0));
    chart.plot("data", (2.0, 5.0));
    chart.plot("data", (6.0, 2.0));

    canvas
}

fn chart_2(root: &rish::TkTopLevel) -> rish::TkCanvas {
    let canvas = std_canvas(root);

    let chart = rish::make_x_y(&canvas, (-20.0, 20.0, 5.0), (0.0, 7.0, 2.0))
        .isometric(true)
        .plot();
    chart.title("Squares appear as squares on the screen (isometric true)", rish::Justify::Centre);
    chart.plot("data", (4.0, 7.0));
    chart.plot("data", (7.0, 7.0));
    chart.plot("data", (7.0, 4.0));
    chart.plot("data", (4.0, 4.0));

    canvas
}

fn chart_3(root: &rish::TkTopLevel) -> rish::TkCanvas {
    let canvas = std_canvas(root);

    let chart = rish::make_histogram(&canvas, (0.0, 10.0, 1.0), (0.0, 10.0, 5.0))
        .x_labels(&["1", "4", "6"])
        .plot();
    chart.title("Histogram with custom labels (xlabels + xconfig)", rish::Justify::Centre);
    chart.x_format("%.0fns");
    chart.plot("data", (0.0, 1.0));
    chart.plot("data", (1.0, 2.0));
    chart.plot("data", (2.0, 5.0));
    chart.plot("data", (6.0, 2.0));

    canvas
}

fn main() {
    let root = rish::start_wish().unwrap();
    root.title("plotdemos19.rs");

    chart_1(&root).pack().layout();
    chart_2(&root).pack().layout();
    chart_3(&root).pack().layout();

    rish::mainloop();
}
