use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();

    let canvas = rish::make_canvas(&root);
    canvas.width(400);
    canvas.height(400);
    canvas.background("white");
    canvas.grid().layout();

    let histogram = rish::make_histogram(&canvas, (0.0, 10.0, 1.0), (0.0, 100.0, 5.0))
        .plot();
    histogram.title("Example Histogram", rish::Justify::Centre);
    histogram.v_title("Cumulative Frequency");
    histogram.x_title("Event");
    histogram.series_colour("data", "green");
    histogram.series_style("data", rish::HistogramStyle::Filled);
    histogram.series_fill_colour("data", "green");

    for i in (2..=10).step_by(2) {
        let i = i as f64;
        histogram.plot_cumulative("data", (i, i*3.0));
    }

    rish::mainloop();
}

