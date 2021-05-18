//! Entry widgets

use super::grid;
use super::widgets;
use super::wish;

/// Refers to an entry widget 
#[derive(Clone)]
pub struct TkEntry {
    pub id: String,
}

pub fn make_entry(parent: &impl widgets::TkWidget) -> TkEntry {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::entry {}", id);
    wish::tell_wish(&msg);

    TkEntry {
        id,
    }
}

super::tkwidget!(TkEntry);

impl TkEntry {
    pub fn width (&self, value: u32) {
        let msg = format!("{} configure -width {{{}}}", self.id, value);
        wish::tell_wish(&msg);
    }
}

