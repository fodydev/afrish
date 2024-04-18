use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();
    root.title("Ternary Diagram from rish");

    let canvas = rish::make_canvas(&root);
    canvas.height(500);
    canvas.width(500);
    canvas.grid().layout();

    let ternary_diagram = rish::make_ternary_diagram(&canvas, false, 5);
    ternary_diagram.corner_titles("Component A", "Component B", "Component C");
    ternary_diagram.plot("data", (50.0, 25.0, 25.0), "1", rish::Direction::West);
    ternary_diagram.plot("data", (20.0, 25.0, 55.0), "2", rish::Direction::East);
    ternary_diagram.draw_line("data", &[(0.0, 80.0, 20.0), (10.0, 20.0, 70.0)]);

    ternary_diagram.series_colour("area", "green");
    ternary_diagram.series_smooth("area", true);
    ternary_diagram.draw_filled_polygon("area", &[(0.0, 70.0, 30.0),
    (10.0, 20.0, 70.0), (0.0, 0.0, 100.0)]);
    ternary_diagram.plot("area1", (0.0, 70.0, 30.0), "", rish::Direction::West);
    ternary_diagram.plot("area1", (10.0, 20.0, 70.0), "", rish::Direction::West);
    ternary_diagram.plot("area1", (0.0, 0.0, 100.0), "", rish::Direction::West);

    ternary_diagram.draw_ticklines("grey");

    rish::mainloop();
}

