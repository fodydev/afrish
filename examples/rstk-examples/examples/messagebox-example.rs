use rish;

fn main() {
    let root = rish::start_wish().unwrap();

    rish::message_box()
        .title("Example message box")
        .message("Click 'OK' to move \nto the next example")
        .show();

    let result = rish::message_box()
        .parent(&root)
        .title("Example 2")
        .message("Check docs")
        .detail("Did you know more docs are on the web?")
        .icon(rish::IconImage::Information)
        .type_buttons(rish::DialogType::YesNo)
        .show();

    println!("You clicked {} on last dialog", result);

    rish::mainloop();
}
