// Translation of plotdemos2.tcl

use rish::*;
use rand;

fn generate_data(s: rish::TkXYPlot, xold: f64, xd: f64, yold: f64, yd: f64) {
    let xnew = xold + xd;
    let ynew = yold + (rand::random::<f64>() - 0.5) * yd;
    let ynew2 = yold + (rand::random::<f64>() - 0.5) * 2.0 * yd;
    s.plot("series1", (xnew, ynew));
    s.plot("series2", (xnew, ynew2));

    if xnew < 200.0 {
        rish::after(500, move || { 
            generate_data(s.clone(), xnew, xd, ynew, yd); 
        });
    }
}

// -- convenience function
fn make_white_canvas(root: &rish::TkTopLevel) -> rish::TkCanvas {
    let canvas = rish::make_canvas(root);
    canvas.background("white");
    canvas.width(400);
    canvas.height(200);
    canvas
}

// strip chart - adding data over time
fn canvas_1(root: &rish::TkTopLevel) -> rish::TkCanvas {
    let canvas = make_white_canvas(root);

    let s = rish::make_strip_chart(&canvas, (0.0, 100.0, 10.0), (0.0, 100.0, 20.0))
        .plot();
    s.title("Aha!", rish::Justify::Centre);
    rish::after(100, move || { generate_data(s.clone(), 0.0, 15.0, 50.0, 30.0); });

    canvas
}

// isometric plot
fn canvas_2(root: &rish::TkTopLevel) -> rish::TkCanvas {
    let canvas = make_white_canvas(root);

    let s = rish::make_isometric_plot(&canvas, (0.0, 100.0), (0.0, 200.0),
                                        rish::StepSize::NoAxes);

    s.rectangle((10.0, 10.0), (50.0, 50.0), "green");
    s.filled_rectangle((20.0, 20.0), (40.0, 40.0), "red");
    s.filled_circle((70.0, 70.0), 40.0, "yellow");
    s.circle((70.0, 70.0), 42.0, "black");

    canvas
}

// different symbols on xy plot
fn canvas_3(root: &rish::TkTopLevel) -> rish::TkCanvas {
    let canvas = make_white_canvas(root);

    let s = rish::make_x_y(&canvas, (0.0, 100.0, 10.0), (0.0, 100.0, 20.0))
        .plot();
    s.series_colour("series1", "red");
    s.series_drawing_mode("series1", rish::DrawingMode::Symbol);
    s.series_colour("series2", "green");
    s.series_drawing_mode("series2", rish::DrawingMode::Both);
    s.y_format("%12.2e");

    let mut x = 5.0;
    for symbol in [rish::Symbol::Circle,
                    rish::Symbol::Cross,
                    rish::Symbol::Dot,
                    rish::Symbol::Down,
                    rish::Symbol::DownFilled,
                    rish::Symbol::Plus,
                    rish::Symbol::Up,
                    rish::Symbol::UpFilled].iter().cloned() {
        s.series_symbol("series1", symbol.clone(), 5);
        s.series_symbol("series2", symbol, 5);
        s.plot("series1", (x, 50.0));
        s.plot("series2", (x, 20.0));
        x += 10.0;
    }

    canvas
}

fn canvas_4(root: &rish::TkTopLevel) -> rish::TkCanvas {
    let canvas = make_white_canvas(root);

    let s = rish::make_x_y(&canvas, (0.0, 100.0, 10.0), (0.0, 100.0, 20.0))
        .plot();
    s.background_gradient_colour("green", 
                                 rish::GradientDirection::TopDown, 
                                 rish::Brightness::Dark);
    s.series_fill_area("series1", rish::FillArea::AboveLine);
    s.series_fill_colour("series1", "white");

    s.plot("series1", (0.0, 20.0));
    s.plot("series1", (10.0, 20.0));
    s.plot("series1", (30.0, 50.0));
    s.plot("series1", (35.0, 45.0));
    s.plot("series1", (45.0, 25.0));
    s.plot("series1", (75.0, 55.0));
    s.plot("series1", (100.0, 55.0));

    s.plaintext((30.0, 60.0), "Peak", rish::Direction::South);

    canvas
}

fn canvas_5(root: &rish::TkTopLevel) -> rish::TkCanvas {
    let canvas = make_white_canvas(root);

    let s = rish::make_x_y(&canvas, (0.0, 100.0, 10.0), (0.0, 100.0, 20.0))
        .plot();
    s.background_gradient_colour("green", 
                                 rish::GradientDirection::TopDown, 
                                 rish::Brightness::Dark);

    s.background_image(&rish::read_image("examples/tcllogo.gif"));
    s.plot("series1", (0.0, 20.0));
    s.plot("series1", (10.0, 20.0));
    s.plot("series1", (30.0, 50.0));
    s.plot("series1", (35.0, 45.0));
    s.plot("series1", (45.0, 25.0));
    s.plot("series1", (75.0, 55.0));
    s.plot("series1", (100.0, 55.0));

    canvas
}

// -- frame 1
fn frame_1(root: &rish::TkTopLevel) { 
    canvas_1(root).grid().row(0).layout();
    canvas_2(root).grid().row(1).layout();
}

// -- frame 2
fn frame_2(root: &rish::TkTopLevel) { 
    let window = rish::make_toplevel(root);
    window.title("plotdemos2.rs - h");

    canvas_3(&window).grid().row(0).layout();
}

// -- frame 3
fn frame_3(root: &rish::TkTopLevel) { 
    let window = rish::make_toplevel(root);
    window.title("plotdemos2.rs - t2");

    canvas_4(&window).grid().row(0).layout();
    canvas_5(&window).grid().row(1).layout();
}

fn main() {
    let root = rish::start_wish().unwrap();
    root.title("plotdemos2.rs");

    frame_1(&root);
    frame_2(&root);
    frame_3(&root);

    rish::mainloop();
}
