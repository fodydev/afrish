//! Combobox widgets
//!
//! A text field with a drop-down list of options
//!
//! * also see the Tk [manual](http://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_combobox.htm)

use super::grid;
use super::widget;
use super::wish;

/// Refers to a combobox widget 
#[derive(Clone)]
pub struct TkCombobox {
    pub id: String,
}

/// Creates an instance of a combo-box widget in given parent, 
/// populating the drop-down list with the given set of values.
pub fn make_combobox(parent: &impl widget::TkWidget, values: &[&str]) -> TkCombobox {
    let id = wish::next_wid(parent.id());
    let mut values_str = String::from("");
    for value in values {
        values_str.push('{');
        values_str.push_str(value);
        values_str.push('}');
        values_str.push(' ');
    }

    let msg = format!("ttk::combobox {} -values {{{}}}", id, values_str);
    wish::tell_wish(&msg);

    TkCombobox {
        id,
    }
}

super::tkwidget!(TkCombobox);
super::tklayouts!(TkCombobox);

impl TkCombobox {
    /// Returns the current value 
    pub fn get_value(&self) -> String {
        let msg = format!("puts [{} get] ; flush stdout", self.id);
        wish::eval_wish(&msg)
    }

    /// Sets the height of the widget, in rows
    pub fn height(&self, value: u32) {
        let msg = format!("{} configure -height {{{}}}", self.id, value);
        wish::tell_wish(&msg);
    }

    /// Alignment of text within widget
    pub fn justify(&self, value: widget::Justify) {
        widget::justify(&self.id, value);
    }

    /// Sets the current value 
    pub fn set_value(&self, value: &str) {
        let msg = format!("puts [{} set {{{}}}] ; flush stdout", self.id, value);
        wish::tell_wish(&msg);
    }

    /// Sets the state of the widget (readonly, normal or disabled).
    pub fn state(&self, value: widget::State) {
        widget::state(&self.id, value);
    }

    /// Sets the width of the widget, in characters
    pub fn width(&self, value: u32) {
        let msg = format!("{} configure -width {{{}}}", self.id, value);
        wish::tell_wish(&msg);
    }
}

