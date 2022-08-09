use rstk::*;

fn main() {
    let root = rstk::start_wish().unwrap();

    root.title("Font Chooser Demo");
    let button = rstk::make_button(&root);
    button.text("Show font chooser");
    button.command(|| { // toggles font chooser visibility
        if rstk::font_chooser_visible() {
            rstk::font_chooser_hide();
        } else {
            rstk::font_chooser_show();
        }
    });
    button.grid().column(0).row(0).layout();

    let label = rstk::make_label(&root);
    label.text("Text Widget 1");
    label.font(&rstk::TkFont {
        family: String::from("Courier"),
        size: 14,
        ..Default::default()
    });
    label.grid().column(0).row(1).layout();

    rstk::font_chooser_parent(&root);
    rstk::font_chooser_command(move |font| { label.font(&font); });

    rstk::mainloop();
}
