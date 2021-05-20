//! Entry widgets

use super::grid;
use super::widgets;
use super::wish;

/// Refers to an entry widget 
#[derive(Clone)]
pub struct TkEntry {
    pub id: String,
    var: String,
}

pub fn make_entry(parent: &impl widgets::TkWidget) -> TkEntry {
    let id = wish::next_wid(parent.id());
    let var = format!("::en{}", wish::current_id());
    let msg = format!("ttk::entry {} -textvariable {}", id, var);
    wish::tell_wish(&msg);

    TkEntry {
        id,
        var,
    }
}

super::tkwidget!(TkEntry);

impl TkEntry {
    /// Returns the current entry value 
    pub fn value(&self) -> String {
        let msg = format!("puts ${} ; flush stdout", self.var);
        wish::eval_wish(&msg)
    }

    /// Sets the width of the widget, in characters
    pub fn width(&self, value: u32) {
        let msg = format!("{} configure -width {{{}}}", self.id, value);
        wish::tell_wish(&msg);
    }
}

