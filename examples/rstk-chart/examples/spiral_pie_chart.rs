use rstk::*;

fn main() {
    let root = rstk::start_wish().unwrap();
    root.title("Spiral Pie Chart in rstk");

    let canvas = rstk::make_canvas(&root);
    canvas.width(500);
    canvas.height(300);
    canvas.background("white");
    canvas.grid().layout();

    let spiral_pie = rstk::make_spiral_pie_chart(&canvas);
    spiral_pie.colours(&["yellow", "blue", "red"]);

    spiral_pie.title("Books Read per Category", rstk::Justify::Centre);
    spiral_pie.plot(&[("Computing", 3.0), ("Fiction", 10.0), ("Technical", 25.0)]);

    rstk::mainloop();
}
