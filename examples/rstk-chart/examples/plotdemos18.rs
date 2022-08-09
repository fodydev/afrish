// Translation of plotdemos18.tcl

use rstk::*;
use rand::Rng;

fn rand(upper: f64) -> f64 {
    rand::thread_rng().gen_range(0.0..upper)
}

fn main() {
    let root = rstk::start_wish().unwrap();
    root.title("plotdemos18.rs");

    let canvas = rstk::make_canvas(&root);
    canvas.width(600);
    canvas.height(400);
    canvas.background("white");
    canvas.grid().layout();

    let devices = vec!["e1", "e2", "e12231", "r1", "r2"];
    let timeline = rstk::make_status_timeline(&canvas, 
                                              (0.0, 7200.0, 900.0),
                                              &devices,
                                              false);
    timeline.title("Operational over time", rstk::Justify::Centre);

    // add randomised data
    let mut last_i = 0.0;
    let mut i = rand(10.0);
    while i < 7200.0 {
        for item in devices.iter() {
            timeline.plot(&item, last_i, i, 
                          if rand(1.0) > 0.5 { "red" } else { "green" });
        }
        last_i = i;
        i += rand(600.0);
    }

    // add labelled vertical lines
    for x in (0..7200).step_by(900) {
        let text = format!("{:02}h:{:02}", x/3600, x % 60);
        if (x % 3600) == 0 {
            timeline.draw_line(&text, x as f64, "black", rstk::ChartDash::Lines, 2.0);
        } else {
            timeline.draw_line("", x as f64, "grey", rstk::ChartDash::Dots3, 2.0);
        }
    }

    rstk::mainloop();
}
