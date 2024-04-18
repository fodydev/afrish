use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();
    root.title("Box Plot in rish");
    let canvas = rish::make_canvas(&root);
    canvas.width(400);
    canvas.height(400);
    canvas.grid().layout();

    let box_plot = rish::make_horizontal_box_plot(&canvas, 
                                                  (0.0, 40.0, 5.0),
                                                  &["A", "B", "C", "D", "E"]);
    box_plot.title("Box plot example", rish::Justify::Left);

    box_plot.plot("data1", "A", &[0.0, 1.0, 2.0, 5.0, 7.0, 1.0, 4.0, 5.0, 0.6, 5.0, 5.5]);
    box_plot.plot("data2", "C", &[2.0, 2.0, 3.0, 6.0, 1.5, 3.0]);
    box_plot.plot("data3", "E", &[2.0, 3.0, 3.0, 4.0, 7.0, 8.0, 9.0, 9.0, 10.0, 10.0, 11.0, 11.0, 11.0, 14.0, 15.0, 17.0, 17.0, 20.0, 24.0, 29.0]);

    rish::mainloop();
}
