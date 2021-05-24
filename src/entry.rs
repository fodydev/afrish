//! Entry widgets
//!
//! A text field for user input or editing.
//!
//! * also see the Tk [manual](http://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_entry.htm)

use super::grid;
use super::widget;
use super::wish;

/// Refers to an entry widget 
#[derive(Clone)]
pub struct TkEntry {
    pub id: String,
    var: String,
}

/// Creates an instance of an entry widget in given parent.
pub fn make_entry(parent: &impl widget::TkWidget) -> TkEntry {
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
super::tklayouts!(TkEntry);

impl TkEntry {
    /// Specifies the font to use for text.
    pub fn font(&self, definition: &str) {
        widget::configure(&self.id, "font", definition);
    }

    /// Specifies the foreground (text) colour.
    pub fn foreground(&self, colour: &str) {
        widget::configure(&self.id, "foreground", colour);
    }

    /// Alignment of text within widget
    pub fn justify(&self, value: widget::Justify) {
        widget::justify(&self.id, value);
    }

    /// Used e.g. for a password, shows the given character instead of 
    /// what is typed.
    pub fn show(&self, c: char) {
        widget::configure(&self.id, "show", &c.to_string());
    }

    /// Sets the state of the widget (readonly, normal or disabled).
    pub fn state(&self, value: widget::State) {
        widget::state(&self.id, value);
    }

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

