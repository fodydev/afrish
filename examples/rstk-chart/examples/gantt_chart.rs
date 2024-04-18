use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();
    root.title("Gantt Chart in rish");

    let canvas = rish::make_canvas(&root);
    canvas.width(400);
    canvas.height(400);
    canvas.background("white");
    canvas.grid().layout();

    let gantt = rish::make_gantt_chart(&canvas, "1 March 2021", "10 April 2021")
        .num_items(5)
        .ylabel_width(15)
        .plot();
    gantt.milestone("Vacation Start", "20 March 2021", "red");
    let task_1 = gantt.task("rust", "1 March 2021", "15 March 2021", 30);
    let task_2 = gantt.task("chart", "15 March 2021", "31 March 2021", 80);
    let task_3 = gantt.task("examples", "7 March 2021", "31 March 2021", 50);
    let task_4 = gantt.task("documentation", "15 March 2021", "31 March 2021", 20);

    gantt.connect(&task_1, &task_2);
    gantt.connect(&task_3, &task_4);

    gantt.draw_line("1 Mar", "1 March 2021", "blue");
    gantt.draw_line("8 Mar", "8 March 2021", "green");
    gantt.draw_line("15 Mar", "15 March 2021", "green",);
    gantt.draw_line("22 Mar", "22 March 2021", "green");
    gantt.draw_line("1 Apr", "1 April 2021", "blue");

    gantt.title("Learning Rust/Tk", rish::Justify::Right);
    gantt.uncompleted_colour("red");

    rish::mainloop();
}

