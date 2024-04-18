// translation of plotdemos5.tcl

use rish::*;

static X: [[f64; 3]; 4] = [
    [0.0, 100.0, 200.0],
    [0.0, 100.0, 200.0],
    [0.0, 100.0, 200.0],
    [0.0, 100.0, 200.0],
];
static Y: [[f64; 3]; 4] = [
    [0.0, 0.0, 0.0],
    [30.0, 30.0, 30.0],
    [60.0, 60.0, 60.0],
    [90.0, 90.0, 90.0],
];
static F: [[f64; 3]; 4] = [
    [0.0, 1.0, 10.0],
    [0.0, 30.0, 30.0],
    [10.0, 60.0, 60.0],
    [30.0, 90.0, 90.0],
];
static CONTOURS: [f64; 21] = [
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
static X_LIMITS: (f64, f64, f64) = (0.0, 200.0, 50.0);
static Y_LIMITS: (f64, f64, f64) = (0.0, 100.0, 20.0);

// -- convenience function
fn make_white_canvas(root: &rish::TkTopLevel) -> rish::TkCanvas {
    let canvas = rish::make_canvas(root);
    canvas.background("white");
    canvas.width(500);
    canvas.height(500);
    canvas
}

fn frame_1(root: &rish::TkTopLevel) {
    let canvas = make_white_canvas(root);
    canvas.grid().layout();

    let chart = rish::make_x_y(&canvas, X_LIMITS, Y_LIMITS).plot();
    rish::colour_map(rish::ColourMap::Jet);
    chart.draw_contour_fill(&X, &Y, &F, &CONTOURS);
    chart.draw_grid(&X, &Y);
}

fn frame_2(root: &rish::TkTopLevel) {
    let window = rish::make_toplevel(root);
    window.title("Contour Demo : contourlines (default colourmap)");

    let canvas = make_white_canvas(&window);
    canvas.grid().layout();

    let chart = rish::make_x_y(&canvas, X_LIMITS, Y_LIMITS).plot();
    chart.draw_contour_lines(&X, &Y, &F, &CONTOURS);
    chart.draw_grid(&X, &Y);
}

fn frame_3(root: &rish::TkTopLevel) {
    let window = rish::make_toplevel(root);
    window.title("Contour Demo : contourlines (hot colourmap)");

    let canvas = make_white_canvas(&window);
    canvas.grid().layout();

    let chart = rish::make_x_y(&canvas, X_LIMITS, Y_LIMITS).plot();
    rish::colour_map(rish::ColourMap::Hot);
    chart.draw_contour_lines(&X, &Y, &F, &CONTOURS);
    chart.draw_grid(&X, &Y);
}

fn frame_4(root: &rish::TkTopLevel) {
    let window = rish::make_toplevel(root);
    window.title("Contour Demo : grey contourfill, jet contourlines");

    let canvas = make_white_canvas(&window);
    canvas.grid().layout();

    let chart = rish::make_x_y(&canvas, X_LIMITS, Y_LIMITS).plot();
    rish::colour_map(rish::ColourMap::Grey);
    chart.draw_contour_fill(&X, &Y, &F, &CONTOURS);
    rish::colour_map(rish::ColourMap::Jet);
    chart.draw_contour_lines(&X, &Y, &F, &CONTOURS);
    chart.draw_grid(&X, &Y);
}

fn frame_5(root: &rish::TkTopLevel) {
    let window = rish::make_toplevel(root);
    window.title("Contour Demo : contourlines (cool colourmap)");

    let canvas = make_white_canvas(&window);
    canvas.grid().layout();

    let chart = rish::make_x_y(&canvas, X_LIMITS, Y_LIMITS).plot();
    rish::colour_map(rish::ColourMap::Cool);
    chart.draw_contour_lines(&X, &Y, &F, &CONTOURS);
    chart.draw_grid(&X, &Y);
}

fn frame_6(root: &rish::TkTopLevel) {
    let window = rish::make_toplevel(root);
    window.title("Contour Demo : default contours (jet colourmap)");

    let canvas = make_white_canvas(&window);
    canvas.grid().layout();

    let chart = rish::make_x_y(&canvas, X_LIMITS, Y_LIMITS).plot();
    rish::colour_map(rish::ColourMap::Jet);
    chart.draw_contour_fill(&X, &Y, &F, &CONTOURS);
    chart.draw_grid(&X, &Y);
}

fn main() {
    let root = rish::start_wish().unwrap();
    root.title("Contour Demo : shade (jet colourmap)");

    frame_1(&root);
    frame_2(&root);
    frame_3(&root);
    frame_4(&root);
    frame_5(&root);
    frame_6(&root);

    rish::mainloop();
}
