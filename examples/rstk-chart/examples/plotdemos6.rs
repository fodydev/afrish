// translation of plotdemos6.tcl

use rstk::*;
use rand::Rng;
use core::f64::consts::PI;

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

fn hypot(x: f64, y: f64) -> f64 {
    (x*x + y*y).sqrt()
}

fn forces_dipole(x: f64, y: f64) -> (f64, f64) {
    let xd1 = 51.0;
    let yd1 = 50.0;
    let xd2 = 49.0;
    let yd2 = 50.0;
    let r1p3 = hypot(x-xd1, y-yd1).powf(3.0);
    let r2p3 = hypot(x-xd2, y-yd2).powf(3.0);
    let fx = ((x-xd1) / r1p3) - ((x-xd2) / r2p3);
    let fy = ((y-yd1) / r1p3) - ((y-yd2) / r2p3);
        
    (fx, fy)
}

// -- frame 1
fn frame_1(root: &rstk::TkTopLevel) {
    let canvas1 = make_white_canvas(root, 400, 400);
    let canvas2 = make_white_canvas(root, 400, 200);
    canvas1.grid().row(0).layout();
    canvas2.grid().row(1).layout();

    let s = rstk::make_x_y(&canvas1, (0.0, 100.0, 10.0), (0.0, 100.0, 20.0)).plot();
    s.vector_colour("series1", "red");
    s.vector_scale("series1", 40.0);
    s.vector_colour("series2", "blue");
    s.vector_scale("series2", 50.0);
    s.vector_type("series2", rstk::CoordinatesType::Nautical);
    s.vector_centred("series2", true);

    // cartesian
    for pair in [(1.0, 0.0), (0.0, 1.0), (0.5, 0.5), (-2.0, 1.0)] {
        s.draw_vector("series1", (30.0, 20.0), pair);
    }
    // nautical
    for pair in [(1.0, 0.0), (1.0, 45.0), (2.0, 90.0)] {
        s.draw_vector("series2", (60.0, 40.0), pair);
    }

    let s = rstk::make_x_y(&canvas2, (0.0, 100.0, 10.0), (0.0, 100.0, 20.0)).plot();
    s.dot_colour("series1", "red");
    s.dot_scale_by_value("series1", true);
    s.dot_scale("series1", 2.5);

    s.dot_colour("series2", "magenta");
    s.dot_scale_by_value("series2", false);
    s.dot_classes("series2", 
                  &[(0.0, "blue"), (1.0, "green"), (2.0, "yellow"), (3.0, "red")]);
    s.dot_outline("series2", false);

    s.dot_colour("series3", "magenta");
    s.dot_scale_by_value("series3", true);
    s.dot_classes("series2", 
                  &[(0.0, "blue"), (1.0, "green"), (2.0, "yellow"), (3.0, "red")]);
    s.dot_outline("series3", true);

    let y1 = 20.0;
    let y2 = 50.0;
    let y3 = 80.0;
    let mut x = 10.0;
    for value in [-1.0, 0.5, 1.5, 2.5, 3.5, 4.5] {
        s.draw_dot("series1", (x, y1), value);
        s.draw_dot("series2", (x, y2), value);
        s.draw_dot("series3", (x, y3), value);
        x += 10.0;
    }

}

// -- frame 2
fn frame_2(root: &rstk::TkTopLevel) {
    let window = rstk::make_toplevel(root);
    window.title("plotdemos6.rs - dipole");

    let canvas = make_white_canvas(&window, 500, 500);
    canvas.grid().layout();

    let s = rstk::make_x_y(&canvas, (45.0, 55.0, 1.0), (45.0, 55.0, 1.0))
        .plot();

    s.title("Forces in a dipole field", rstk::Justify::Centre);
    s.vector_colour("series1", "black");
    s.vector_scale("series1", 40.0);
    s.vector_type("series1", rstk::CoordinatesType::Polar);
    s.dot_colour("dipole", "red");
    s.dot_scale_by_value("dipole", false);
    s.dot_radius("dipole", 5.0);
    s.draw_dot("dipole", (49.0, 50.0), 1.0);
    s.draw_dot("dipole", (51.0, 50.0), 1.0);

    let mut y = 45.25;
    while y <= 55.0 {
        let mut x = 43.25;
        while x <= 55.0 {
            let (u, v) = forces_dipole(x, y);
            // scale vector for better display
            s.draw_vector("series1",
                          (x, y),
                          ((0.5 + hypot(u, v)) / (1.0 + hypot(u, v)),
                           (180.0 * v.atan2(u)) / PI));
            x += 0.5;
        }
        y += 0.5;
    }
}

// -- frame 3
fn frame_3(root: &rstk::TkTopLevel) {
    let window = rstk::make_toplevel(root);
    window.title("plotdemos6.rs - rchart");

    let canvas = make_white_canvas(&window, 400, 200);
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

}

fn main() {
    let root = rstk::start_wish().unwrap();
    root.title("plotdemos6.rs");

    frame_1(&root);
    frame_2(&root);
    frame_3(&root);

    rstk::mainloop();
}
