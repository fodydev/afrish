use afrish;

fn main() {
    let root = afrish::start_wish().unwrap();

    root.title("Menu Example");
    let menubar = afrish::make_menu(&root);

    // -- create file menu
    let file_menu = afrish::make_menu(&menubar);
    file_menu.command()
        .label("New")
        .command(|| println!("You clicked 'New'"))
        .add();
    file_menu.command()
        .label("Save")
        .state(afrish::State::Disabled)
        .add();
    file_menu.separator().add();
    file_menu.command()
        .label("Quit")
        .underline(0)
        .accelerator("Ctrl-Q")
        .command(|| afrish::end_wish())
        .add();

    // -- create example menu
    let example_menu = afrish::make_menu(&menubar);
    example_menu.check_button()
        .label("Select")
        .command(|value| println!("Selection is now {}", value))
        .add();
    let colours_menu = afrish::make_menu(&example_menu);
    colours_menu.radio_button("colours", "red")
        .label("Red")
        .add();
    colours_menu.radio_button("colours", "blue")
        .label("Blue")
        .add();
    colours_menu.radio_button("colours", "green")
        .label("Green")
        .add();
    example_menu.cascade()
        .menu(&colours_menu)
        .label("Colours")
        .add();
    colours_menu.radio_button_value("colours", "blue");

    example_menu.separator().add();

    example_menu.command()
        .label("Show")
        .command(move || println!("Colour is: {}",
                                  colours_menu.radio_button_value_get("colours")) )
        .add();

    // -- combine menus into menubar
    root.menu(&menubar);
    menubar.cascade().menu(&file_menu).label("File").add();
    menubar.cascade().menu(&example_menu).label("Example").add();

    afrish::mainloop();
}

