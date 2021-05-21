use rstk;

fn main() {
    let root = rstk::start_wish();

    root.title("frame-example.rs");
    let frame_1 = rstk::make_frame(&root);
    let frame_2 = rstk::make_frame(&root);
    frame_2.border_width(2);
    frame_2.relief(rstk::Relief::Sunken);
    let frame_3 = rstk::make_frame(&root);
    frame_3.border_width(2);
    frame_3.relief(rstk::Relief::Groove);

    frame_1.grid().row(0).column(0).padx(5).pady(5).layout();
    frame_2.grid().row(0).column(1).padx(5).pady(5).layout();
    frame_3.grid().row(0).column(2).padx(5).pady(5).layout();

    frame_1.padding(&[2]);
    frame_2.padding(&[5, 40, 10, 10]);

    let label_1 = rstk::make_label(&frame_1);
    label_1.text("A Label");
    label_1.grid().layout();

    let label_2 = rstk::make_label(&frame_2);
    label_2.text("A Label");
    label_2.grid().layout();

    let label_3 = rstk::make_label(&frame_3);
    label_3.text("A Label");
    label_3.grid().layout();

    rstk::mainloop();
}

