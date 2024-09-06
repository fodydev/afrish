use afrish::*;

fn main() {
    let root = afrish::start_wish().unwrap();
    root.title("font-example-2.rs");

    let mut fonts = afrish::font_families();
    fonts.sort();
    println!("There are {} font families, e.g. \n{}\n{}\n{}",
             fonts.len(), fonts[0], fonts[1], fonts[200]);
    let font = afrish::TkFont {
        family: fonts[1].clone(),
        size: 24,
        ..Default::default()
    };

    let label = afrish::make_label(&root);
    label.text(&format!("font: {} 24pt\n{}", fonts[1], font.metrics()));
    label.font(&font);
    label.grid().layout();

    afrish::mainloop();
}

