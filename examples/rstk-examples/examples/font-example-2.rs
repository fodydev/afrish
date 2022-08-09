use rstk::*;

fn main() {
    let root = rstk::start_wish().unwrap();
    root.title("font-example-2.rs");

    let mut fonts = rstk::font_families();
    fonts.sort();
    println!("There are {} font families, e.g. \n{}\n{}\n{}",
             fonts.len(), fonts[0], fonts[1], fonts[200]);
    let font = rstk::TkFont {
        family: fonts[1].clone(),
        size: 24,
        ..Default::default()
    };

    let label = rstk::make_label(&root);
    label.text(&format!("font: {} 24pt\n{}", fonts[1], font.metrics()));
    label.font(&font);
    label.grid().layout();

    rstk::mainloop();
}

