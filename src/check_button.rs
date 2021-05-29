//! Check buttons 
//!
//! A label-like on/off widget, which executes a command when clicked.
//!
//! * also see the Tk [manual](https://tcl.tk/man/tcl/TkCmd/ttk_checkbutton.htm)
//!

use super::grid;
use super::widget;
use super::wish;

/// Refers to a check-button widget
#[derive(Clone,Debug,PartialEq)]
pub struct TkCheckButton {
    pub id: String,
    var: String,
}

/// Creates an instance of a check-button widget in given parent.
pub fn make_check_button(parent: &impl widget::TkWidget) -> TkCheckButton {
    let id = wish::next_wid(parent.id());
    let var = format!("::cb{}", wish::current_id());
    let msg = format!("set {} 0 ; ttk::checkbutton {} -variable {}", var, id, var);
    wish::tell_wish(&msg);

    TkCheckButton {
        id,
        var,
    }
}

impl widget::TkWidget for TkCheckButton {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}
impl grid::TkGridLayout for TkCheckButton {
}
impl widget::TkLabelOptions for TkCheckButton {
}

impl TkCheckButton {
    /// Sets the function to be called when the button is clicked.
    pub fn command (&self, command: impl Fn(bool)->() + Send + 'static) {
        wish::add_callback1_bool(&self.id, wish::mk_callback1_bool(command));
        let msg = format!("{} configure -command {{ puts cb1b-{}-${} ; flush stdout }}", 
                          self.id, self.id, self.var);
        wish::tell_wish(&msg);
    }

    /// Calls the button's command.
    pub fn invoke(&self) {
        let msg = format!("{} invoke", self.id);
        wish::tell_wish(&msg);
    }

    /// Returns true/false if button is selected (checked) or not.
    pub fn is_selected(&self) -> bool {
        let msg = format!("puts ${} ; flush stdout", self.var);
        let result = wish::eval_wish(&msg); 
        result == "1"
    }

    /// Sets the selected (checked) state 
    pub fn selected(&self, value: bool) {
        let msg = format!("set {} {}", self.var, if value { "1" } else { "0" });
        wish::tell_wish(&msg);
    }

    /// Sets the state of the button (normal or disabled).
    pub fn state(&self, value: widget::State) {
        widget::configure(&self.id, "state", &value.to_string());
    }
}
