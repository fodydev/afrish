use super::wish;

pub trait TkWidget {
    fn id(&self) -> &str;
}

// --------------------------------------------------------------------------
// Enums to type-check values

pub enum Compound {
    Bottom,
    Center,
    Centre,
    Image,
    Left,
    None,
    Right,
    Text,
    Top,
}

pub enum Relief {
    Flat,
    Groove,
    Raised,
    Ridge,
    Solid,
    Sunken,
}

pub enum State {
    Disabled,
    Normal,
}

// --------------------------------------------------------------------------
// Internal functions for within crate use

pub(super) fn compound (wid: &str, value: Compound) {
    let value = match value {
        Compound::Bottom => "bottom",
        Compound::Center | Compound::Centre => "center",
        Compound::Image => "image",
        Compound::Left => "left",
        Compound::None => "none",
        Compound::Right => "right",
        Compound::Text => "text",
        Compound::Top => "top",
    };
    configure(wid, "compound", value);
}

pub(super) fn state (wid: &str, value: State) {
    let value = match value {
        State::Disabled => "disabled",
        State::Normal => "normal",
    };
    configure(wid, "state", value);
}

pub(super) fn configure(wid: &str, option: &str, value: &str) {
    let msg = format!("{} configure -{} {{{}}}", wid, option, value);
    wish::tell_wish(&msg);
}

pub(super) fn focus(wid: &str) {
    let msg = format!("focus {}", wid);
    wish::tell_wish(&msg);
}

pub(super) fn grid_configure(wid: &str, option: &str, value: &str) {
    let msg = format!("grid configure {} -{} {{{}}}", wid, option, value);
    wish::tell_wish(&msg);
}

pub(super) fn grid_configure_column(wid: &str, index: u32, option: &str, value: &str) {
    let msg = format!("grid columnconfigure {} {} -{} {{{}}}", wid, index, option, value);
    wish::tell_wish(&msg);
}

pub(super) fn grid_configure_row(wid: &str, index: u32, option: &str, value: &str) {
    let msg = format!("grid rowconfigure {} {} -{} {{{}}}", wid, index, option, value);
    wish::tell_wish(&msg);
}


