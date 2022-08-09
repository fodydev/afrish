use rstk::*;

fn main() {
    let root = rstk::start_wish().unwrap();
    root.title("3D Bar Chart");

    let canvas = rstk::make_canvas(&root);
    canvas.width(400);
    canvas.height(400);
    canvas.background("white");
    canvas.grid().layout();

    let bar_3d = rstk::make_3d_bar_chart(&canvas, (0.0, 60.0, 5.0), 7);

    bar_3d.title("Number of Moons per Planet", rstk::Justify::Centre);
    bar_3d.label_font(&rstk::TkFont {
        family: String::from("Times"),
        size: 8,
        ..Default::default()
    });
    bar_3d.show_background(true);

    bar_3d.plot("Earth", 1.0, "blue");
    bar_3d.plot("Mars", 2.0, "red");
    bar_3d.plot("Jupiter", 53.0, "orange");
    bar_3d.plot("Saturn", 53.0, "yellow");
    bar_3d.plot("Uranus", 27.0, "green");
    bar_3d.plot("Neptune", 13.0, "cyan");
    bar_3d.plot("Pluto", 5.0, "grey");

    rstk::mainloop();
}
