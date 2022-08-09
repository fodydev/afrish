use rstk::*;

fn main() {
    let root = rstk::start_wish().unwrap();
    root.title("tx_plot example");

    let canvas = rstk::make_canvas(&root);
    canvas.width(500);
    canvas.height(200);
    canvas.background("white");
    canvas.grid().layout();

    let tx_plot = rstk::make_tx(&canvas,
                                ("2001-01-01", "2015-01-01", 1461),
                                (-10.0, 20.0, 10.0))
        .plot();

    tx_plot.series_colour("min", "red");
    tx_plot.series_colour("max", "blue");

    tx_plot.x_title("Time");
    tx_plot.v_title("Temperature");

    tx_plot.legend_position(rstk::Position::BottomRight);
    tx_plot.legend("min", "Minimum Temperature");
    tx_plot.legend("max", "Maximum Temperature");

    tx_plot.plot("min", ("2001-01-01", -3.0));
    tx_plot.plot("min", ("2004-01-01", 4.0));
    tx_plot.plot("min", ("2007-01-01", 2.0));
    tx_plot.plot("min", ("2010-01-01", -1.0));
    tx_plot.plot("min", ("2013-01-01", 2.0));
    tx_plot.plot("min", ("2014-01-01", 5.0));

    tx_plot.plot("max", ("2001-01-01", 10.0));
    tx_plot.plot("max", ("2004-01-01", 12.0));
    tx_plot.plot("max", ("2007-01-01", 8.0));
    tx_plot.plot("max", ("2010-01-01", 6.0));
    tx_plot.plot("max", ("2013-01-01", 15.0));
    tx_plot.plot("max", ("2014-01-01", 18.0));

    rstk::mainloop();
}

