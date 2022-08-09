use rstk::*;

fn main() {
    let root = rstk::start_wish().unwrap();
    root.title("plotdemos8.rs");
    let canvas = rstk::make_canvas(&root);
    canvas.grid().layout();

    let box_plot = rstk::make_horizontal_box_plot(&canvas, 
                                                  (0.0, 40.0, 5.0),
                                                  &["A", "B", "C", "D", "E"]);

    box_plot.plot("data1", "A", &[0.0, 1.0, 2.0, 5.0, 7.0, 1.0, 4.0, 5.0, 0.6, 5.0, 5.5]);
    box_plot.plot("data2", "C", &[2.0, 2.0, 3.0, 6.0, 1.5, 3.0]);
    box_plot.plot("data3", "E", &[2.0, 3.0, 3.0, 4.0, 7.0, 8.0, 9.0, 9.0, 10.0, 10.0, 11.0, 11.0, 11.0, 14.0, 15.0, 17.0, 17.0, 20.0, 24.0, 29.0]);

    rstk::mainloop();
}
