// Translation of plotdemos4.tcl

use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();
    root.title("plotdemos4.rs");

    // -- 3D barchart in first frame
    let canvas = rish::make_canvas(&root);
    canvas.width(400);
    canvas.height(400);
    canvas.background("white");
    canvas.grid().layout();

    let chart = rish::make_3d_bar_chart(&canvas, (-200.0, 900.0, 100.0), 7);

    for (colour, value) in &[("red", 765.0), ("green", 234.0), ("blue", 345.0),
                             ("yellow", 321.0), ("magenta", 567.0), 
                             ("cyan", -123.0), ("white", 400.0)] {
        chart.plot(colour, *value, colour);
    }
    chart.title("3D Bars", rish::Justify::Centre);
    chart.balloon((1.2, 100.0), 
                  "Arrow pointing\nat second bar", 
                  rish::Direction::SouthEast);

    // -- three styles of radial charts in their own window
    let toplevel = rish::make_toplevel(&root);
    toplevel.title("plotdemos4.rs - t");

    for style in &[rish::RadialStyle::Lines, 
                    rish::RadialStyle::Cumulative, 
                    rish::RadialStyle::Filled] {
        let canvas = rish::make_canvas(&toplevel);
        canvas.width(300);
        canvas.height(200);
        canvas.background("white");
        canvas.pack().fill(rish::PackFill::Both).layout();
        let chart = rish::make_radial_chart(&canvas,
                                            &["A", "B", "LongerName", "C", "D"],
                                            10.0,
                                            style.clone());
        chart.plot(&[1.0, 2.0, 3.0, 4.0, 3.0], "green", 2);
        chart.plot(&[4.0, 5.0, 0.0, 1.0, 4.0], "red", 3);
        chart.title(&format!("Sample of a radial chart - style {}", style),
                    rish::Justify::Centre);
    }

    rish::mainloop();
}

