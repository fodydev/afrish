use afrish::*;

const TEXT: &str = "Rust has great documentation, a friendly compiler with useful error messages, and top-notch tooling - an integrated package manager and build tool, smart multi-editor support with auto-completion and type inspections, an auto-formatter, and more. -- https://rust-lang.org";

fn main() {
    let root = afrish::start_wish().unwrap();

    root.title("text-example-1.rs");
    root.grid_configure_column(0, "weight", "1");
    root.grid_configure_row(0, "weight", "1");

    let text_1 = afrish::make_text(&root);
    text_1.width(30);
    text_1.height(20);
    text_1.insert_end(&TEXT);
    text_1.wrap(afrish::Wrapping::Word);
    text_1.grid().row(0).column(0).sticky(afrish::Sticky::NESW).layout();

    let text_2 = afrish::make_text(&root);
    text_2.width(30);
    text_2.height(20);
    text_2.insert_end(&TEXT);
    text_2.state(afrish::State::Disabled);
    text_2.grid().row(0).column(1).sticky(afrish::Sticky::NESW).layout();

    afrish::mainloop();
}

