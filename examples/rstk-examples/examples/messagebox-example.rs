use rstk;

fn main() {
    let root = rstk::start_wish().unwrap();

    rstk::message_box()
        .title("Example message box")
        .message("Click 'OK' to move \nto the next example")
        .show();

    let result = rstk::message_box()
        .parent(&root)
        .title("Example 2")
        .message("Check docs")
        .detail("Did you know more docs are on the web?")
        .icon(rstk::IconImage::Information)
        .type_buttons(rstk::DialogType::YesNo)
        .show();

    println!("You clicked {} on last dialog", result);

    rstk::mainloop();
}
