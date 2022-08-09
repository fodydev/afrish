use rstk::*;

fn main() {
    let root = rstk::start_wish().unwrap();
    root.title("plotdemos3.rs");

    let canvas = rstk::make_canvas(&root);
    canvas.width(500);
    canvas.height(200);
    canvas.background("white");
    canvas.grid().layout();

    let gantt = rstk::make_gantt_chart(&canvas, "1 january 2004", "31 december 2004")
        .num_items(4)
        .plot();
    let task_from = gantt.task("Spring", ("1 march 2004", "1 june 2004"), 30);
    let task_to = gantt.task("Summer", ("1 june 2004", "1 september 2004"), 80);

    gantt.summary("First half", &[&task_from, &task_to]);
    gantt.connect(&task_from, &task_to);

    gantt.draw_line("1 jan", "1 january 2004", "green");
    gantt.draw_line("1 apr", "1 april 2004", "green");
    gantt.draw_line("1 jul", "1 july 2004", "green",);
    gantt.draw_line("1 oct", "1 october 2004", "green");

    gantt.milestone("Longest day", "21 july 2004", "blue");
    gantt.title("Seasons (northern hemisphere)", rstk::Justify::Centre);

    // make copies
    let window = rstk::make_toplevel(&root);
    window.title("plotdemos3.rs - .t.c");
    let canvas = rstk::make_canvas(&window);
    canvas.width(700);
    canvas.height(500);
    canvas.grid().layout();

    rstk::plot_pack(&canvas, rstk::PlotDirection::Top, &[&gantt, &gantt]);
    rstk::plot_pack(&canvas, rstk::PlotDirection::Left, &[&gantt]);

    rstk::mainloop();
}

