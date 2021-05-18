use rstk;

fn main() {
    let root = rstk::start_wish();

    root.title("label-example.rs");
    let label_1 = rstk::make_label(&root);
    label_1.text("Simple text label");
    
    let image = rstk::make_image("examples/tcllogo.gif");

    let label_2 = rstk::make_label(&root);
    label_2.image(&image);

    let label_3 = rstk::make_label(&root);
    label_3.font("Helvetical 14 bold");
    label_3.text("Some text\non multiple\nlines");

    let label_4 = rstk::make_label(&root);
    label_4.image(&image);
    label_4.text("Tcl Logo");

    let label_5 = rstk::make_label(&root);
    label_5.image(&image);
    label_5.text("Tcl Logo");

    let label_6 = rstk::make_label(&root);
    label_6.image(&image);
    label_6.text("Tcl Logo");

    let text = "Rust has great documentation, a friendly compiler with useful error messages, and top-notch tooling - an integrated package manager and build tool, smart multi-editor support with auto-completion and type inspections, an auto-formatter, and more. -- https://rust-lang.org";

    let label_7 = rstk::make_label(&root);
    label_7.text(text);
    label_7.wrap_length(300);

    label_4.compound(rstk::Compound::Bottom);
    label_5.compound(rstk::Compound::Centre);
    label_6.compound(rstk::Compound::Top);

    label_1.grid().row(0).column(0).layout();
    label_2.grid().row(0).column(1).layout();
    label_3.grid().row(0).column(2).layout();
    label_4.grid().row(1).column(0).layout();
    label_5.grid().row(1).column(1).layout();
    label_6.grid().row(1).column(2).layout();
    label_7.grid().row(2).column(0).column_span(3).sticky("news").layout();

    rstk::mainloop();
}
