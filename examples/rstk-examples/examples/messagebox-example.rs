use afrish;

fn main() {
    let root = afrish::start_wish().unwrap();

    afrish::message_box()
        .title("Example message box")
        .message("Click 'OK' to move \nto the next example")
        .show();

    let result = afrish::message_box()
        .parent(&root)
        .title("Example 2")
        .message("Check docs")
        .detail("Did you know more docs are on the web?")
        .icon(afrish::IconImage::Information)
        .type_buttons(afrish::DialogType::YesNo)
        .show();

    println!("You clicked {} on last dialog", result);

    afrish::mainloop();
}
