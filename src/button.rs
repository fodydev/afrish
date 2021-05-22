//! Button widgets
//!
//! A label-like widget, which executes a command when clicked.
//!
//! * also see the Tk [manual](http://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_button.htm)

use super::grid;
use super::image;
use super::widgets;
use super::wish;

/// Refers to a button widget 
#[derive(Clone)]
pub struct TkButton {
    pub id: String,
}

/// Creates an instance of a button widget in given parent.
pub fn make_button(parent: &impl widgets::TkWidget) -> TkButton {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::button {}", id);
    wish::tell_wish(&msg);

    TkButton {
        id,
    }
}

super::tkwidget!(TkButton);
super::tklabelfunctions!(TkButton);

impl TkButton {
    /// Sets the function to be called when the button is clicked.
    pub fn command(&self, command: impl Fn()->() + 'static) {
        wish::add_callback0(&self.id, wish::mk_callback0(command));
        let msg = format!("{} configure -command {{ puts clicked-{} ; flush stdout }}", self.id, self.id);
        wish::tell_wish(&msg);
    }

    /// Calls the button's command.
    pub fn invoke(&self) {
        let msg = format!("{} invoke", self.id);
        wish::tell_wish(&msg);
    }

    /// Sets the state of the button (normal or disabled).
    pub fn state(&self, value: widgets::State) {
        widgets::state(&self.id, value);
    }
}

