use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();
    root.title("Horizontal Bar Chart from rish");

    let canvas = rish::make_canvas(&root);
    canvas.height(400);
    canvas.width(400);
    canvas.background("white");
    canvas.grid().layout();

    let bar_chart = rish::make_horizontal_bar_chart(&canvas,
                                                    (0.0, 50.0, 5.0),
                                                    &["2018", "2019", "2020", "2021"],
                                                    rish::BarSeries::Stacked);
    bar_chart.title("Book Reading History", rish::Justify::Centre);
    bar_chart.x_title("Number of Books");
    bar_chart.v_title("Year");

    bar_chart.show_values(true);
    bar_chart.plot("type-1", &[3.0, 6.0, 9.0, 12.0], "blue"); 
    bar_chart.plot("type-2", &[10.0, 7.0, 11.0, 15.0], "red"); 
    bar_chart.plot("type-3", &[25.0, 18.0, 21.0, 22.0], "green");

    bar_chart.legend_position(rish::Position::BottomRight);
    bar_chart.legend_spacing(12);
    bar_chart.legend("type-1", "Computing");
    bar_chart.legend("type-2", "Fiction");
    bar_chart.legend("type-3", "Technical");

    rish::mainloop();
}
