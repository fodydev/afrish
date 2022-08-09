use rstk::*;
use rand::Rng;

fn main() {
    let root = rstk::start_wish().unwrap();
    root.title("xy_plot example 2 - plotdemos6: rchart");

    let canvas = rstk::make_canvas(&root);
    canvas.width(400);
    canvas.height(200);
    canvas.background("white");
    canvas.grid().layout();

    let chart = rstk::make_x_y(&canvas, (0.0, 100.0, 10.0), (0.0, 50.0, 10.0))
        .plot();
    chart.title("R-chart (arbitrary data)", rstk::Justify::Centre);

    chart.series_colour("series1", "green");
    for x in (1..50).step_by(3) {
        let y = 20.0 + rand::thread_rng().gen_range(1.0..3.0);
        chart.rchart("series1", (x as f64, y));
    }
    // now some data outside the expected range
    chart.rchart("series1", (50.0, 41.0));
    chart.rchart("series1", (52.0, 42.0));
    chart.rchart("series1", (54.0, 39.0));

    // and continue with the well-behaved series
    for x in (57..100).step_by(3) {
        let y = 20.0 + rand::thread_rng().gen_range(1.0..3.0);
        chart.rchart("series1", (x as f64, y));
    }

    rstk::mainloop();
}
