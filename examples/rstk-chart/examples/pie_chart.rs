use rstk::*;

fn main() {
    let root = rstk::start_wish().unwrap();
    root.title("Pie Chart in rstk");

    let canvas = rstk::make_canvas(&root);
    canvas.width(500);
    canvas.height(300);
    canvas.background("white");
    canvas.grid().layout();

    let pie_chart = rstk::make_pie_chart(&canvas);
    pie_chart.title("Books Read per Category", rstk::Justify::Centre);
    pie_chart.plot(&[("Computing", 3.0), ("Fiction", 10.0), ("Technical", 25.0)]);
    pie_chart.explode(0);

    rstk::mainloop();
}
