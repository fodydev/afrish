use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();
    root.title("Radial Chart in rish");

    let canvas = rish::make_canvas(&root);
    canvas.width(400);
    canvas.height(400);
    canvas.background("white");
    canvas.grid().layout();

    let radial_chart = rish::make_radial_chart(&canvas, 
                                               &["Mon", "Tue", "Wed", "Thu", "Fri"],
                                               10.0,
                                               rish::RadialStyle::Lines);
    radial_chart.plot(&[5.0, 8.0, 4.0, 7.0, 10.0], "green", 2);
    radial_chart.plot(&[2.0, 4.0, 1.0, 3.0, 5.0], "blue", 2);

    rish::mainloop();
}

