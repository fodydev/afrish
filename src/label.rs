//! Label widgets

use super::grid;
use super::image;
use super::widgets;
use super::wish;

/// Refers to a label widget 
#[derive(Clone)]
pub struct TkLabel {
    pub id: String,
}

pub fn make_label(parent: &impl widgets::TkWidget) -> TkLabel {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::label {}", id);
    wish::tell_wish(&msg);

    TkLabel {
        id,
    }
}

super::tkwidget!(TkLabel);
super::tklabelfunctions!(TkLabel);

impl TkLabel {
    pub fn wrap_length(&self, len: u32) {
        widgets::configure(&self.id, "wraplength", &len.to_string());
    }
}

