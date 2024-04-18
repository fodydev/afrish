use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();
    root.title("xy_plot example 1");

    let canvas = rish::make_canvas(&root);
    canvas.width(400);
    canvas.height(400);
    canvas.background("white");
    canvas.grid().layout();

    let xy = rish::make_x_y(&canvas, (-10.0, 10.0, 2.0), (-100.0, 100.0, 20.0))
        .plot();
    xy.title("Two Functions", rish::Justify::Centre);
    xy.x_title("Input x");
    xy.v_title("Output y");
    xy.series_colour("square", "blue");
    xy.series_colour("cube", "green");

    xy.legend_position(rish::Position::BottomRight);
    xy.legend("square", "x*x");
    xy.legend("cube", "x*x*x");

    for n in 0..20 {
        let i = (n-10) as f64;
        xy.plot("square", (i, i*i));
        xy.plot("cube", (i, i*i*i));
    }

    xy.balloon((0.0, 0.0), "crossover point", rish::Direction::NorthWest);
    xy.plaintext_colour("red");
    xy.plaintext((6.0, 80.0), "diverging", rish::Direction::NorthWest);
    xy.save("xy-plot.ps");

    rish::mainloop();
}
