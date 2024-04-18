use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();
    root.title("Bar Chart from rish");

    let canvas = rish::make_canvas(&root);
    canvas.height(400);
    canvas.width(400);
    canvas.background("white");
    canvas.grid().layout();

    let bar_chart = rish::make_bar_chart(&canvas,
                                         &["2018", "2019", "2020", "2021"],
                                         (0.0, 30.0, 5.0),
                                         rish::BarSeries::Count(3),
                                         0.0);
    bar_chart.title("Book Reading History", rish::Justify::Centre);
    bar_chart.x_title("Year");
    bar_chart.v_title("Number of Books");

    bar_chart.show_values(true);
    bar_chart.plot("person-1", &[3.0, 6.0, 9.0, 12.0], "blue"); 
    bar_chart.plot("person-2", &[10.0, 7.0, 11.0, 15.0], "red"); 
    bar_chart.plot("person-3", &[25.0, 18.0, 21.0, 22.0], "green");

    bar_chart.legend_position(rish::Position::TopRight);
    bar_chart.legend("person-1", "Mary");
    bar_chart.legend("person-2", "Peter");
    bar_chart.legend("person-3", "Shalini");

    rish::mainloop();
}
