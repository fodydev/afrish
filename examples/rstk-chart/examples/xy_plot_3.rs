use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();
    root.title("xy_plot example 3 - from plotdemos5");

    let canvas = rish::make_canvas(&root);
    canvas.width(500);
    canvas.height(500);
    canvas.background("white");
    canvas.grid().layout();

    let x = [
        [0.0, 100.0, 200.0],
        [0.0, 100.0, 200.0],
        [0.0, 100.0, 200.0],
        [0.0, 100.0, 200.0],
    ];
    let y = [
        [0.0, 0.0, 0.0],
        [30.0, 30.0, 30.0],
        [60.0, 60.0, 60.0],
        [90.0, 90.0, 90.0],
    ];
    let f = [
        [0.0, 1.0, 10.0],
        [0.0, 30.0, 30.0],
        [10.0, 60.0, 60.0],
        [30.0, 90.0, 90.0],
    ];
    let contours = [
        0.0,
        5.2631578947,
        10.5263157895,
        15.7894736842,
        21.0526315789,
        26.3157894737,
        31.5789473684,
        36.8421052632,
        42.1052631579,
        47.3684210526,
        52.6315789474,
        57.8947368421,
        63.1578947368,
        68.4210526316,
        73.6842105263,
        78.9473684211,
        84.2105263158,
        89.4736842105,
        94.7368421053,
        100.0,
        105.263157895,
    ];
    let x_limits = (0.0, 200.0, 50.0);
    let y_limits = (0.0, 100.0, 20.0);

    let chart = rish::make_x_y(&canvas, x_limits, y_limits).plot();
    chart.title(
        "Contour Demo: contourlines (default colourmap)",
        rish::Justify::Centre,
    );
    chart.draw_contour_fill(&x, &y, &f, &contours);
    chart.draw_grid(&x, &y);

    rish::mainloop();
}
