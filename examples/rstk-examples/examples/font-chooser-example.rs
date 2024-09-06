use afrish::*;

fn main() {
    let root = afrish::start_wish().unwrap();

    root.title("Font Chooser Demo");
    let button = afrish::make_button(&root);
    button.text("Show font chooser");
    button.command(|| { // toggles font chooser visibility
        if afrish::font_chooser_visible() {
            afrish::font_chooser_hide();
        } else {
            afrish::font_chooser_show();
        }
    });
    button.grid().column(0).row(0).layout();

    let label = afrish::make_label(&root);
    label.text("Text Widget 1");
    label.font(&afrish::TkFont {
        family: String::from("Courier"),
        size: 14,
        ..Default::default()
    });
    label.grid().column(0).row(1).layout();

    afrish::font_chooser_parent(&root);
    afrish::font_chooser_command(move |font| { label.font(&font); });

    afrish::mainloop();
}
