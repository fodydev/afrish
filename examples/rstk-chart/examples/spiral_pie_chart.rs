use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();
    root.title("Spiral Pie Chart in rish");

    let canvas = rish::make_canvas(&root);
    canvas.width(500);
    canvas.height(300);
    canvas.background("white");
    canvas.grid().layout();

    let spiral_pie = rish::make_spiral_pie_chart(&canvas);
    spiral_pie.colours(&["yellow", "blue", "red"]);

    spiral_pie.title("Books Read per Category", rish::Justify::Centre);
    spiral_pie.plot(&[("Computing", 3.0), ("Fiction", 10.0), ("Technical", 25.0)]);

    rish::mainloop();
}
