use rish::*;

fn main() {
    let root = rish::start_wish().unwrap();

    root.title("Font Chooser Demo");
    let button = rish::make_button(&root);
    button.text("Show font chooser");
    button.command(|| { // toggles font chooser visibility
        if rish::font_chooser_visible() {
            rish::font_chooser_hide();
        } else {
            rish::font_chooser_show();
        }
    });
    button.grid().column(0).row(0).layout();

    let label = rish::make_label(&root);
    label.text("Text Widget 1");
    label.font(&rish::TkFont {
        family: String::from("Courier"),
        size: 14,
        ..Default::default()
    });
    label.grid().column(0).row(1).layout();

    rish::font_chooser_parent(&root);
    rish::font_chooser_command(move |font| { label.font(&font); });

    rish::mainloop();
}
