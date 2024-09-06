use afrish::*;

const TEXT: &str = "Rust has great documentation, a friendly compiler with useful error messages, and top-notch tooling - an integrated package manager and build tool, smart multi-editor support with auto-completion and type inspections, an auto-formatter, and more. -- https://rust-lang.org";

fn main() {
    let root = afrish::start_wish().unwrap();
    root.title("text-example-2.rs");

    let text = afrish::make_text(&root);
    text.width(40);
    text.height(5);
    text.wrap(afrish::Wrapping::None);
    for _ in 0..10 {
        text.insert_end(&TEXT);
        text.insert_end("\n");
    }

    let hscroll = afrish::make_horizontal_scrollbar(&root, &text);
    let vscroll = afrish::make_vertical_scrollbar(&root, &text);

    text.grid().column(0).row(0).sticky(afrish::Sticky::NESW).layout();
    hscroll.grid().column(0).row(1).sticky(afrish::Sticky::EW).layout();
    vscroll.grid().column(1).row(0).sticky(afrish::Sticky::NS).layout();
    root.grid_configure_column(0, "weight", "1");
    root.grid_configure_row(0, "weight", "1");

    text.see((6, 30)); // starts with line 6, character 30 on display

    afrish::mainloop();
}

