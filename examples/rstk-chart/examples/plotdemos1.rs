// Translation of plotdemos1.tcl

use rstk::*;
use rand;

// -- convenience function
fn make_white_canvas(root: &rstk::TkTopLevel,
                     width: u64,
                     height: u64) -> rstk::TkCanvas {
    let canvas = rstk::make_canvas(root);
    canvas.background("white");
    canvas.width(width);
    canvas.height(height);
    canvas
}

// XY plot
fn canvas_1(root: &rstk::TkTopLevel) -> rstk::TkCanvas {
    let canvas = make_white_canvas(root, 400, 200);
    let s = rstk::make_x_y(&canvas, (0.0, 100.0, 10.0), (0.0, 100.0, 20.0))
        .plot();
    let r = rstk::make_right_axis(&canvas, (0.0, 0.1, 0.01));
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
    s.title("Aha!", rstk::Justify::Centre);

    // some data for the right axis
    r.series_drawing_mode("right", rstk::DrawingMode::Both);
    r.series_colour("right", "green");
    r.plot("right", (10.0, 0.01));
    r.plot("right", (30.0, 0.03));
    r.plot("right", (40.0, 0.02));

    canvas
}

// Pie chart
fn canvas_2(root: &rstk::TkTopLevel) -> rstk::TkCanvas {
    let canvas = make_white_canvas(root, 400, 200);
    
    let s = rstk::make_pie_chart(&canvas);
    s.plot(&[("Long names", 10.0), ("Short names", 30.0),
             ("Average", 40.0), ("Ultra-short names", 5.0)]);
    s.title("Okay - this works", rstk::Justify::Centre);

    canvas
}

// Polar plot
fn canvas_3(root: &rstk::TkTopLevel) -> rstk::TkCanvas {
    let canvas = make_white_canvas(root, 400, 200);

    let s = rstk::make_polar(&canvas, (3.0, 1.0));
    for angle in (0..360).step_by(10) {
        let angle = angle as f64;
        let rad = 1.0 + (std::f64::consts::PI * angle/180.0).cos();
        s.plot("cardioid", (rad, angle));
    }

    s.title("Cardioid", rstk::Justify::Centre);

    canvas
}

// Time series plot
fn canvas_4(root: &rstk::TkTopLevel) -> rstk::TkCanvas {
    let canvas = make_white_canvas(root, 400, 200);
    
    let s = rstk::make_tx(&canvas, ("2006-01-01", "2007-01-01", 120),
                          (0.0, 100.0, 20.0)).plot();
    s.series_colour("series1", "red");
    s.series_colour("series2", "blue");

    s.x_title("Time");
    s.y_title("Data");
    s.x_tick_lines("green", rstk::ChartDash::Dots2);

    s.plot("series1", ("2006-02-01", 10.0));
    s.plot("series1", ("2006-02-11", 50.0));
    s.plot("series1", ("2006-03-01", 50.0));
    s.plot("series1", ("2006-07-01", 40.0));
    s.plot("series1", ("2006-08-21", 20.0));
    s.plot("series1", ("2006-08-22", 1.0));
    s.plot("series1", ("2006-12-11", 78.0));

    s.plot("series2", ("2006-03-01", 110.0));
    s.plot("series2", ("2006-04-11", 50.0));
    s.plot("series2", ("2006-07-28", 20.0));
    s.plot("series2", ("2006-10-21", 99.0));
    s.plot("series2", ("2006-11-22", 1.0));
    s.plot("series2", ("2006-12-31", 78.0));

    canvas
}

// Bar chart with two series
fn canvas_5(root: &rstk::TkTopLevel) -> rstk::TkCanvas {
    let canvas = make_white_canvas(root, 400, 200);

    let s = rstk::make_bar_chart(&canvas, &["A", "B", "C", "D", "E"],
                                 (0.0, 10.0, 2.0),
                                 rstk::BarSeries::Count(2), 
                                 0.0);
    s.plot("series1", &[1.0, 4.0, 6.0, 1.0, 7.0], "red");
    s.plot("series2", &[0.0, 3.0, 7.0, 9.3, 2.0], "green");

    s.title("Arbitrary data", rstk::Justify::Centre);
    s.legend_spacing(20);
    s.legend("series1", "Series 1");
    s.legend("series2", "Series 2");

    canvas
}

// Stacked bar chart
fn canvas_6(root: &rstk::TkTopLevel) -> rstk::TkCanvas {
    let canvas = make_white_canvas(root, 400, 200);

    let s = rstk::make_bar_chart(&canvas, &["A", "B", "C", "D", "E"],
                                 (0.0, 20.0, 5.0),
                                 rstk::BarSeries::Stacked, 
                                 0.0);
    s.plot("series1", &[1.0, 4.0, 6.0, 1.0, 7.0], "red");
    s.plot("series2", &[0.0, 3.0, 7.0, 9.3, 2.0], "green");

    s.title("Stacked diagram", rstk::Justify::Centre);

    canvas
}

// Horizontal bar chart - with two series
fn canvas_7(root: &rstk::TkTopLevel) -> rstk::TkCanvas {
    let canvas = make_white_canvas(root, 400, 200);

    let s = rstk::make_horizontal_bar_chart(&canvas,
                                            (0.0, 10.0, 2.0),
                                            &["A", "B", "C", "D", "E"],
                                            rstk::BarSeries::Count(2));
    s.plot("series1", &[1.0, 4.0, 6.0, 1.0, 7.0], "red");
    s.plot("series2", &[0.0, 3.0, 7.0, 9.3, 2.0], "green");

    s.title("Arbitrary data", rstk::Justify::Centre);

    canvas
}

// Horizontal bar chart - stacked
fn canvas_8(root: &rstk::TkTopLevel) -> rstk::TkCanvas {
    let canvas = make_white_canvas(root, 400, 200);

    let s = rstk::make_horizontal_bar_chart(&canvas,
                                            (0.0, 20.0, 5.0),
                                            &["A", "B", "C", "D", "E"],
                                            rstk::BarSeries::Stacked);
    s.plot("series1", &[1.0, 4.0, 6.0, 1.0, 7.0], "red");
    s.plot("series2", &[0.0, 3.0, 7.0, 9.3, 2.0], "green");

    s.title("Stacked diagram", rstk::Justify::Centre);

    canvas
}

// Time chart
fn canvas_9(root: &rstk::TkTopLevel) -> rstk::TkCanvas {
    let canvas = make_white_canvas(root, 400, 200);

    let s = rstk::make_time_chart(&canvas,
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

    s.title("Seasons (northern hemisphere)", rstk::Justify::Centre);

    canvas
}

fn square(n: f64) -> f64 { n*n }
fn cowboyhat(x: f64, y: f64) -> f64 {
    let x1 = x/9.0;
    let y1 = y/9.0;

    3.0 * (1.0 - (square(x1) + square(y1))) * (1.0 - (square(x1) + square(y1)))
}

// 3D plot
fn canvas_10(root: &rstk::TkTopLevel) -> rstk::TkCanvas {
    let mut data: Vec<Vec<f64>> = vec![];
    for r in (-10..=10).step_by(2) {
        let r = r as f64;
        let mut row = vec![];

        for c in 0..=10 {
            let c = c as f64;
            row.push(cowboyhat(r, c));
        }
        data.push(row);
    }

    let canvas = make_white_canvas(root, 400, 300);

    let s = rstk::make_3d_plot(&canvas,
                               (0.0, 10.0, 3.0),
                               (-10.0, 10.0, 10.0),
                               (0.0, 10.0, 2.5));
    s.title("3D Plot", rstk::Justify::Centre);
    s.plot_data(&data);

    canvas
}

// 3D plot - interpolated data
fn canvas_11(root: &rstk::TkTopLevel) -> rstk::TkCanvas {
    let canvas = make_white_canvas(root, 400, 250);

    let s = rstk::make_3d_plot(&canvas,
                               (0.0, 10.0, 3.0),
                               (-10.0, 10.0, 10.0),
                               (0.0, 10.0, 2.5));
    s.title("3D Plot - data", rstk::Justify::Centre);
    s.colours("green", "black");
    s.plot_data(&[[1.0, 2.0, 1.0, 0.0], 
                [1.1, 3.0, 1.1, -0.5], 
                [3.0, 1.0, 4.0, 5.0]]);

    canvas
}

// 3D plot
fn canvas_12(root: &rstk::TkTopLevel) -> rstk::TkCanvas {
    let canvas = make_white_canvas(root, 400, 250);

    let s = rstk::make_3d_plot_with_labels(&canvas,
                                           (0.0, 10.0, 3.0),
                                           (-10.0, 10.0, 10.0),
                                           (0.0, 10.0, 2.5),
                                           &["A", "B", "C"]);
    s.title("3D Plot - data", rstk::Justify::Centre);
    s.colours("green", "black");
    s.interpolate_data(&[[1.0, 2.0, 1.0, 0.0], 
                       [1.1, 3.0, 1.1, -0.5], 
                       [3.0, 1.0, 4.0, 5.0]],
                       &[0.0, 0.5, 1.0, 1.5, 2.0]);
    canvas
}

// -- frame 1
fn frame_1(root: &rstk::TkTopLevel) { 
    canvas_1(root).grid().row(0).layout();
    canvas_2(root).grid().row(1).layout();
    canvas_3(root).grid().row(2).layout();
    canvas_4(root).grid().row(3).layout();
}

// -- frame 2
fn frame_2(root: &rstk::TkTopLevel) {
    let window = rstk::make_toplevel(root);
    window.title("plotdemos1.rs - h");

    canvas_5(&window).grid().row(0).layout();
    canvas_6(&window).grid().row(1).layout();
}

// -- frame 3
fn frame_3(root: &rstk::TkTopLevel) {
    let window = rstk::make_toplevel(root);
    window.title("plotdemos1.rs - v");

    canvas_7(&window).grid().row(0).layout();
    canvas_8(&window).grid().row(1).layout();
    canvas_9(&window).grid().row(2).layout();
}

// -- frame 4
fn frame_4(root: &rstk::TkTopLevel) {
    let window = rstk::make_toplevel(root);
    window.title("plotdemos1.rs - h3");

    canvas_10(&window).grid().row(0).layout();
    canvas_11(&window).grid().row(1).layout();
    canvas_12(&window).grid().row(2).layout();
}

fn main() {
    let root = rstk::start_wish().unwrap();
    root.title("plotdemos1.rs");

    frame_1(&root);
    frame_2(&root);
    frame_3(&root);
    frame_4(&root);

    rstk::mainloop();
}
