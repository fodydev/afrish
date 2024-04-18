// Translation of plotdemos7.tcl

use rish::*;
use rand;

// -- convenience function
fn make_white_canvas(root: &rish::TkTopLevel,
                     width: u64,
                     height: u64) -> rish::TkCanvas {
    let canvas = rish::make_canvas(root);
    canvas.background("white");
    canvas.width(width);
    canvas.height(height);
    canvas
}

// XY plot
fn canvas_1(root: &rish::TkTopLevel) -> rish::TkCanvas {
    // customise the xy-plot
    // -- plot-config not implemented, so make raw calls
    rish::tell_wish("::Plotchart::plotconfig xyplot title font {Times 14}");
    rish::tell_wish("::Plotchart::plotconfig xyplot title textcolor red");
    rish::tell_wish("::Plotchart::plotconfig xyplot leftaxis font {Helvetica 10 italic}");
    rish::tell_wish("::Plotchart::plotconfig xyplot leftaxis thickness 2");
    rish::tell_wish("::Plotchart::plotconfig xyplot leftaxis ticklength -5");
    rish::tell_wish("::Plotchart::plotconfig xyplot rightaxis font {Times 10 bold}");
    rish::tell_wish("::Plotchart::plotconfig xyplot rightaxis color green");
    rish::tell_wish("::Plotchart::plotconfig xyplot rightaxis thickness 2");
    rish::tell_wish("::Plotchart::plotconfig xyplot margin right 100");

    let canvas = make_white_canvas(root, 400, 200);
    let s = rish::make_x_y(&canvas, (0.0, 100.0, 10.0), (0.0, 100.0, 20.0))
        .plot();
    let r = rish::make_right_axis(&canvas, (0.0, 0.1, 0.01));
    s.series_colour("series1", "red");
    s.series_colour("series2", "blue");
    s.series_colour("series3", "magenta");

    let xd = 5.0;
    let yd = 20.0;
    let mut xold = 0.0;
    let mut yold = 50.0;

    for _ in 0..20 {
        let xnew = xold + xd;
        let ynew = yold + (rand::random::<f64>() - 0.5) * yd;
        let ynew2 = yold + (rand::random::<f64>() - 0.5) * 2.0 * yd;
        s.plot("series1", (xnew, ynew));
        s.plot("series2", (xnew, ynew2));
        s.trend_line("series3", (xnew, ynew));
        xold = xnew;
        yold = ynew;
    }

    s.draw_interval_with_symbol("series2", 50.0, 40.0, 60.0, 52.0);
    s.draw_interval("series2", 60.0, 40.0, 60.0);

    s.x_title("X-coordinate");
    s.y_title("Y-data");
    r.y_title("Right axis");
    s.title("Aha!", rish::Justify::Centre);

    // some data for the right axis
    r.series_drawing_mode("right", rish::DrawingMode::Both);
    r.series_colour("right", "green");
    r.plot("right", (10.0, 0.01));
    r.plot("right", (30.0, 0.03));
    r.plot("right", (40.0, 0.02));

    canvas
}

// Pie chart
fn canvas_2(root: &rish::TkTopLevel) -> rish::TkCanvas {
    let canvas = make_white_canvas(root, 400, 200);
    
    let s = rish::make_pie_chart(&canvas);
    s.plot(&[("Long names", 10.0), ("Short names", 30.0),
             ("Average", 40.0), ("Ultra-short names", 5.0)]);
    s.title("Okay - this works", rish::Justify::Centre);

    canvas
}

// Polar plot
fn canvas_3(root: &rish::TkTopLevel) -> rish::TkCanvas {
    let canvas = make_white_canvas(root, 400, 200);

    let s = rish::make_polar(&canvas, (3.0, 1.0));
    for angle in (0..360).step_by(10) {
        let angle = angle as f64;
        let rad = 1.0 + (std::f64::consts::PI * angle/180.0).cos();
        s.plot("cardioid", (rad, angle));
    }

    s.title("Cardioid", rish::Justify::Centre);

    canvas
}

// Bar chart with two series
fn canvas_5(root: &rish::TkTopLevel) -> rish::TkCanvas {
    let canvas = make_white_canvas(root, 400, 200);

    let s = rish::make_bar_chart(&canvas, &["A", "B", "C", "D", "E"],
                                 (0.0, 10.0, 2.0),
                                 rish::BarSeries::Count(2), 
                                 0.0);
    s.plot("series1", &[1.0, 4.0, 6.0, 1.0, 7.0], "red");
    s.plot("series2", &[0.0, 3.0, 7.0, 9.3, 2.0], "green");

    s.title("Arbitrary data", rish::Justify::Centre);
    s.legend_spacing(20);
    s.legend("series1", "Series 1");
    s.legend("series2", "Series 2");

    canvas
}

// Stacked bar chart
fn canvas_6(root: &rish::TkTopLevel) -> rish::TkCanvas {
    let canvas = make_white_canvas(root, 400, 200);

    let s = rish::make_bar_chart(&canvas, &["A", "B", "C", "D", "E"],
                                 (0.0, 20.0, 5.0),
                                 rish::BarSeries::Stacked, 
                                 0.0);
    s.plot("series1", &[1.0, 4.0, 6.0, 1.0, 7.0], "red");
    s.plot("series2", &[0.0, 3.0, 7.0, 9.3, 2.0], "green");

    s.title("Stacked diagram", rish::Justify::Centre);

    canvas
}

// Horizontal bar chart - with two series
fn canvas_7(root: &rish::TkTopLevel) -> rish::TkCanvas {
    let canvas = make_white_canvas(root, 400, 200);

    let s = rish::make_horizontal_bar_chart(&canvas,
                                            (0.0, 10.0, 2.0),
                                            &["Antarctica", "Eurasia", "The Americas", "Australia and Oceania", "Ocean"],
                                            rish::BarSeries::Count(2));

    s.plot_gradient("series1", &[1.0, 4.0, 6.0, 1.0, 7.0], "red",
                    rish::GradientDirection::LeftRight, rish::Brightness::Bright);
    s.plot_gradient("series2", &[0.0, 3.0, 7.0, 9.3, 2.0], "green",
                    rish::GradientDirection::RightLeft, rish::Brightness::Bright);

    s.title("Arbitrary data", rish::Justify::Centre);

    canvas
}

// Horizontal bar chart - stacked
fn canvas_8(root: &rish::TkTopLevel) -> rish::TkCanvas {
    let canvas = make_white_canvas(root, 400, 200);

    let s = rish::make_horizontal_bar_chart(&canvas,
                                            (0.0, 20.0, 5.0),
                                            &["A", "B", "C", "D", "E"],
                                            rish::BarSeries::Stacked);
    s.plot_gradient("series1", &[1.0, 4.0, 6.0, 1.0, 7.0], "red", 
                    rish::GradientDirection::LeftRight, rish::Brightness::Bright);
    s.plot("series2", &[0.0, 3.0, 7.0, 9.3, 2.0], "green");

    s.title("Stacked diagram", rish::Justify::Centre);

    canvas
}

// Time chart
fn canvas_9(root: &rish::TkTopLevel) -> rish::TkCanvas {
    let canvas = make_white_canvas(root, 400, 200);

    let s = rish::make_time_chart(&canvas,
                                  "1 january 2004",
                                  "31 december 2004")
        .num_items(4)
        .plot();

    s.period("Spring", ("1 march 2004", "1 june 2004"), "green");
    s.period("Summer", ("1 june 2004", "1 september 2004"), "yellow");
    s.draw_line("1 jan", "1 january 2004", "black");
    s.draw_line("1 mar", "1 april 2004", "black");
    s.draw_line("1 jul", "1 july 2004", "black");
    s.draw_line("1 oct", "1 october 2004", "black");
    s.milestone("Longest day", "21 july 2004", "black");

    s.title("Seasons (northern hemisphere)", rish::Justify::Centre);

    canvas
}

// -- frame 1
fn frame_1(root: &rish::TkTopLevel) { 
    canvas_1(root).grid().row(0).layout();
    canvas_2(root).grid().row(1).layout();
    canvas_3(root).grid().row(2).layout();
}

// -- frame 2
fn frame_2(root: &rish::TkTopLevel) {
    let window = rish::make_toplevel(root);
    window.title("plotdemos7.rs - h");

    canvas_5(&window).grid().row(0).layout();
    canvas_6(&window).grid().row(1).layout();
}

// -- frame 3
fn frame_3(root: &rish::TkTopLevel) {
    let window = rish::make_toplevel(root);
    window.title("plotdemos7.rs - v");

    // - use plotconfig to set up display for all horizontal bar-charts
    rish::tell_wish("::Plotchart::plotconfig horizbars leftaxis font {Helvetica 10 italic}");
    rish::tell_wish("::Plotchart::plotconfig horizbars background outercolor steelblue3");
    rish::tell_wish("::Plotchart::plotconfig horizbars bottomaxis ticklength -5");
    
    canvas_7(&window).grid().row(0).layout();
    canvas_8(&window).grid().row(1).layout();
    canvas_9(&window).grid().row(2).layout();
}

fn main() {
    let root = rish::start_wish().unwrap();
    root.title("plotdemos7.rs");

    frame_1(&root);
    frame_2(&root);
    frame_3(&root);

    rish::mainloop();
}
