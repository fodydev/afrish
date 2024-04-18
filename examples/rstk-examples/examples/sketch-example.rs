use rish::*;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    let root = rish::start_wish().unwrap();

    let colour = Arc::new(Mutex::new("blue"));
    let last_x = Arc::new(Mutex::new(0));
    let last_y = Arc::new(Mutex::new(0));

    root.title("sketch-example.rs");
    let canvas = rish::make_canvas(&root);
    canvas.width(500);
    canvas.height(500);
    canvas.background("gray75");

    canvas.grid().column(0).row(0).sticky(rish::Sticky::NESW).layout();
    root.grid_configure_column(0, "weight", "1");
    root.grid_configure_row(0, "weight", "1");

    // add three rectangles, and option to change colour
    let rect_1 = canvas.create_rectangle((10, 10), (30, 30));
    rect_1.fill("red");
    {
        let colour = colour.clone();
        rect_1.bind("<1>", move |_| {
            let mut colour = colour.lock().unwrap();
            *colour = "red";
        });
    }

    let rect_2 = canvas.create_rectangle((10, 35), (30, 55));
    rect_2.fill("blue");
    {
        let colour = colour.clone();
        rect_2.bind("<1>", move |_| {
            let mut colour = colour.lock().unwrap();
            *colour = "blue";
        });
    }

    let rect_3 = canvas.create_rectangle((10, 60), (30, 80));
    rect_3.fill("black");
    {
        let colour = colour.clone();
        rect_3.bind("<1>", move |_| {
            let mut colour = colour.lock().unwrap();
            *colour = "black";
        });
    }

    {
        let canvasc = canvas.clone();
        canvas.bind("<B1-ButtonRelease>", move |_| {
            canvasc.configure_tag("currentline", "width", "1");
        });
    }

    {
        let last_x = last_x.clone();
        let last_y = last_y.clone();
        canvas.bind("<1>", move |event| {
            let mut last_x = last_x.lock().unwrap();
            *last_x = event.x as u64;
            let mut last_y = last_y.lock().unwrap();
            *last_y = event.y as u64;
        });
    }
    
    {
        let colour = colour.clone();
        let last_x = last_x.clone();
        let last_y = last_y.clone();
        canvas.clone().bind("<B1-Motion>", move |event| {
            let colour = colour.lock().unwrap();
            let mut last_x = last_x.lock().unwrap();
            let mut last_y = last_y.lock().unwrap();

            let line = canvas.create_line(&[(*last_x, *last_y), (event.x as u64, event.y as u64)]);
            line.colour(*colour);
            line.width(5);
            line.add_tag("currentline");
            
            *last_x = event.x as u64;
            *last_y = event.y as u64;
        });
    }

    rish::mainloop();
}


