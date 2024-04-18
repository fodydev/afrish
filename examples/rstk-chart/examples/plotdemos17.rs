// translation of plotdemos17.tcl

use rish::*;
use core::f64::consts::PI;

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

fn main() {
    let root = rish::start_wish().unwrap();
    root.title("plotdemos17.rs");

    let c1 = make_white_canvas(&root, 400, 300);
    let c2 = make_white_canvas(&root, 400, 300);
    let c3 = make_white_canvas(&root, 400, 200);

    c1.grid().row(0).layout();
    c2.grid().row(1).layout();
    c3.grid().row(2).layout();

    let p1 = rish::make_x_y(&c1, (0.0, 10.0, 2.0), (-1.0, 1.0, 0.25)).plot();
    let p2 = rish::make_x_y(&c2, (0.0, 10.0, 2.0), (-10.0, 10.0, 5.0)).plot();
    p1.y_title("(V)");
    p1.x_tick_length(20);
    p1.x_format("%.3f");
    p1.y_tick_length(20);
    p1.y_minor_ticks(4);
    p1.y_label_offset(10.0);

    p2.y_title("(mA)");
    p2.x_tick_length(6);
    p2.x_minor_ticks(1);
    p2.series_colour("current", "red");

    // fill in some data
    for i in 0..100 {
        let i = i as f64;
        let phase = (2.0 * i * PI) / 50.0;
        p1.plot("voltage", (phase, 0.9 * phase.cos()));
        p2.plot("current", (phase, 7.0 * phase.sin()));
    }

    // third plot uses xlabels and ylabels
    rish::make_x_y(&c3, (0.0, 10.0, 0.0), (-10.0, 10.0, 0.0))
        .x_labels(&["1", "4", "6"])
        .y_labels(&["-5", "0"])
        .plot();

    rish::mainloop();
}
