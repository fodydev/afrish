use rstk;

fn main() {
    let root = rstk::start_wish();

    root.title("Feet to Metres");
    let content = rstk::make_frame(&root);
    content.configure("padding", "3 3 12 12");
    content.grid()
        .column(0)
        .row(0)
        .sticky("nsew")
        .layout();
    root.grid_configure_column(0, "weight", "1");
    root.grid_configure_row(0, "weight", "1");

    // top row has the entry widget and explanatory label to its right
    let feet_entry = rstk::make_entry(&content);
    feet_entry.width(7);
    feet_entry.grid().column(2).row(1).sticky("we").padx(5).pady(5).layout();

    let feet_label = rstk::make_label(&content);
    feet_label.text("feet");
    feet_label.grid().column(3).row(1).sticky("w").padx(5).pady(5).layout();

    // middle row has three labels
    let desc_label = rstk::make_label(&content);
    desc_label.text("is equivalent to");
    desc_label.grid().column(1).row(2).sticky("e").padx(5).pady(5).layout();

    let result_label = rstk::make_label(&content);
    result_label.text("");
    result_label.grid().column(2).row(2).sticky("we").padx(5).pady(5).layout();
    
    let metres_label = rstk::make_label(&content);
    metres_label.text("metres");
    metres_label.grid().column(3).row(2).sticky("we").padx(5).pady(5).layout();

    let calculate = {
        let feet_entry = feet_entry.clone();

        move || {
            let entry_value = feet_entry.value();
            if let Ok(feet) = entry_value.parse::<f32>() {
                let metres = (0.3048 * feet * 10000.0) / 10000.0;
                result_label.text(&format!("{:.5}", metres.to_string()));
            }
        }
    };

    // last row has the button on right
    let calc_button = rstk::make_button(&content);
    calc_button.text("Calculate");
    calc_button.command(calculate.clone());
    calc_button.grid().column(3).row(3).sticky("w").padx(5).pady(5).layout();

    feet_entry.focus();
    root.bind("<Return>", calculate); 

    rstk::mainloop();
}

