use rstk::*;

const COUNTRY_CODES: &[&str] = &[
"ar", "au", "be", " br", "ca", "cn", "dk", "fi", "fr", "gr", "in", "it", "jp", "mx", "nl", "no",
"es", "se", "ch"
];

const COUNTRY_NAMES: &[&str] = &[
    "Argentina",
    "Australia",
    "Belgium",
    "Brazil",
    "Canada",
    "China",
    "Denmark",
    "Finland",
    "France",
    "Greece",
    "India",
    "Italy",
    "Japan",
    "Mexico",
    "Netherlands",
    "Norway",
    "Spain",
    "Sweden",
    "Switzerland",
];

const COUNTRY_POPN: &[u32] = &[
    41000000, 21179211, 10584534, 185971537,
    33148682, 1323128240, 5457415, 5302000, 
    64102140, 11147000, 1131043000, 59206382,
    127718000, 106535000, 16402414, 4738085,
    45116894, 9174082, 7508700
];

fn gift_name(gift: &str) -> String {
    match gift {
        "card" => String::from("Greeting Card"),
        "flowers" => String::from("Flowers"),
        "nastygram" => String::from("Nastygram"),
        _ => String::from("Nothing"),
    }
}

fn send_gift(selection: &[u32],
             gift: &str,
             sent_label: &rstk::TkLabel) {
    if selection.len() == 1 {
        let index = selection[0] as usize;
        let country = COUNTRY_NAMES[index];
        let gift = gift_name(gift);

        sent_label.text(&format!("Sent {} to leader of {}", &gift, country));
    }
}

fn show_population(selection: &[u32], 
                   status_label: &rstk::TkLabel, 
                   sent_label: &rstk::TkLabel) {
    if selection.len() == 1 {
        let index = selection[0] as usize;
        let code = COUNTRY_CODES[index];
        let name = COUNTRY_NAMES[index];
        let popn = COUNTRY_POPN[index];

        status_label.text(&format!("The population of {} ({}) is {}",
        name, code, popn));
        sent_label.text("");
    }
}

fn main() {
    let root = rstk::start_wish().unwrap();

    root.title("Listbox Example: Gift Sending");
    // - outer content frame and widgets
    let content = rstk::make_frame(&root);
    let countries = rstk::make_listbox(&root, &COUNTRY_NAMES);
    let send_label = rstk::make_label(&root);
    send_label.text("Send to country's leader:");
    let gift_1 = rstk::make_radio_button(&root, "gift", "card");
    gift_1.text(&gift_name("card")); 
    let gift_2 = rstk::make_radio_button(&root, "gift", "flowers");
    gift_2.text(&gift_name("flowers")); 
    let gift_3 = rstk::make_radio_button(&root, "gift", "nastygram");
    gift_3.text(&gift_name("nastygram"));
    let sent_label = rstk::make_label(&root);
    sent_label.anchor(rstk::Anchor::Centre);
    let status_label = rstk::make_label(&root);
    status_label.anchor(rstk::Anchor::W);
    let send = rstk::make_button(&root);
    send.text("Send Gift");

    {
        let countries = countries.clone();
        let gift_1 = gift_1.clone();
        let sent_label = sent_label.clone();
        send.command(move || {
            send_gift(&countries.selected_items(), &gift_1.value_get(), &sent_label);
        });
    }

    // - grid the outer content frame
    content.padding(&[5, 5, 12, 0]);
    content.grid().column(0).row(0)
        .sticky(rstk::Sticky::NESW).layout();
    root.grid_configure_column(0, "weight", "1");
    root.grid_configure_row(0, "weight", "1");
    content.grid_configure_column(0, "weight", "1");
    content.grid_configure_row(0, "weight", "1");
    // - grid the other widgets
    countries.grid().row(0).column(0).row_span(6)
        .sticky(rstk::Sticky::NESW).layout();
    send_label.grid().row(0).column(1).padx(10).pady(10).layout();
    gift_1.grid().row(1).column(1).sticky(rstk::Sticky::W).padx(20).layout();
    gift_2.grid().row(2).column(1).sticky(rstk::Sticky::W).padx(20).layout();
    gift_3.grid().row(3).column(1).sticky(rstk::Sticky::W).padx(20).layout();
    send.grid().row(4).column(2).padx(10).sticky(rstk::Sticky::E).layout();
    sent_label.grid().row(5).column(1).column_span(2)
        .padx(5).pady(5).sticky(rstk::Sticky::N).layout();
    status_label.grid().row(6).column(0).column_span(2)
        .sticky(rstk::Sticky::EW).layout();

    gift_1.value("card");
    show_population(&countries.selected_items(), &status_label, &sent_label);

    // - alternate colours in listbox
    for i in 0..COUNTRY_NAMES.len() {
        if i % 2 == 0 {
            countries.item_configure(i as u32, "background", "#f0f0ff");
        }
    }

    // - set event bindings for when selection in listbox changes, 
    // when user double-clicks ths list, and when they hit the Return key.
    {
        let countriesc = countries.clone();
        let sent_label = sent_label.clone();
        countries.bind("<<ListboxSelect>>", move |_| {
            show_population(&countriesc.selected_items(), &status_label, &sent_label);
        });
    }
    {
        let countriesc = countries.clone();
        let gift_1 = gift_1.clone();
        let sent_label = sent_label.clone();
        countries.bind("<Double-1>", move |_| { 
            send_gift(&countriesc.selected_items(), &gift_1.value_get(), &sent_label);
        });
    }
    {
        let countries = countries.clone();
        let gift_1 = gift_1.clone();
        let sent_label = sent_label.clone();
        root.bind("<Return>", move |_| { 
            send_gift(&countries.selected_items(), &gift_1.value_get(), &sent_label);
        });
    }

    rstk::mainloop();
}
