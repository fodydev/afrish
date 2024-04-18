use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();

    root.title("popup-menu-example.rs");

    let menu = rish::make_menu(&root);
    for item in ["One", "Two", "Three"].iter() {
        menu.command()
            .label(item)
            .command(move || println!("You clicked {}", item))
            .add();
    }

    root.bind("<3>", move |event| {
        menu.popup(event.root_x, event.root_y);
    });

    rish::mainloop();
}
