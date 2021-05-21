//! Radio buttons 
//!
//! A label-like on/off widget which can be arranged in a group of mutually 
//! exclusive buttons. Button also executes a command when clicked.
//!
//! * also see the Tk [manual](https://tcl.tk/man/tcl/TkCmd/ttk_radiobutton.htm)
//!

use super::grid;
use super::image;
use super::widgets;
use super::wish;

/// Refers to a radio-button widget
#[derive(Clone)]
pub struct TkRadioButton {
    pub id: String,
    var: String,
}

/// Creates an instance of a radio-button widget in given parent.
/// Radio buttons are arranged in groups, with each button in the group
/// having its own value. For this reason, the group and value are 
/// required when making a radio button.
pub fn make_radio_button(parent: &impl widgets::TkWidget, group: &str, value: &str) -> TkRadioButton {
    let id = wish::next_wid(parent.id());
    let var = format!("::rb_group_{}", group);
    let msg = format!("ttk::radiobutton {} -value {} -variable {}", 
                      id, value, var);
    wish::tell_wish(&msg);

    TkRadioButton {
        id,
        var,
    }
}

super::tkwidget!(TkRadioButton);
super::tklabelfunctions!(TkRadioButton);

impl TkRadioButton {
    /// Sets the function to be called when the button is clicked.
    pub fn command (&self, command: impl Fn(bool)->() + 'static) {
        wish::add_callback1_bool(&self.id, wish::mk_callback1_bool(command));
        let msg = format!("{} configure -command {{ puts cb1-{}-${} ; flush stdout }}", self.id, self.id, self.var);
        wish::tell_wish(&msg);
    }

    /// Sets the state of the button (normal or disabled).
    pub fn state(&self, value: widgets::State) {
        widgets::state(&self.id, value);
    }

    /// Returns the selected value from this radio-button's group.
    pub fn value(&self) -> String {
        let msg = format!("puts ${} ; flush stdout", self.var);
        wish::eval_wish(&msg)
    }
    
    pub fn value_set(&self, value: &str) {
        let msg = format!("set {} {}", self.var, value);
        wish::tell_wish(&msg);
    }
}
