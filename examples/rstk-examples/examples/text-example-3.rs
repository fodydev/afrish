use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();
    root.title("text-example-3.rs");

    let text = rish::make_text(&root);
    text.width(40);
    text.height(5);
    text.wrap(rish::Wrapping::None);
    text.insert_end("abcd\nefgh");

    text.grid().column(0).row(0).sticky(rish::Sticky::NESW).layout();

    //text.delete_char((1,2));
    //text.delete((1,1), (1,2));
    //text.delete((1,0), (2,0));
    //text.delete((1,2), (2,1));
    text.replace((1,2), (2,1), "--");
    println!("Text now: {}", text.get_to_end((1, 0)));

    rish::mainloop();
}

