//! Button widget - displays text/image. Executes a command when clicked.
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_button.htm)

use super::grid;
use super::widget;
use super::wish;

/// Refers to a button widget
#[derive(Clone, Debug, PartialEq)]
pub struct TkButton {
    pub id: String,
}

/// Creates an instance of a button widget in given parent.
pub fn make_button(parent: &impl widget::TkWidget) -> TkButton {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::button {}", id);
    wish::tell_wish(&msg);

    TkButton { id }
}

impl widget::TkWidget for TkButton {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl grid::TkGridLayout for TkButton {}

impl widget::TkLabelOptions for TkButton {}

impl TkButton {
    /// Sets the function to be called when the button is clicked.
    pub fn command(&self, command: impl Fn() + Send + 'static) {
        wish::add_callback0(&self.id, wish::mk_callback0(command));
        let msg = format!(
            "{} configure -command {{ puts clicked-{} ; flush stdout }}",
            self.id, self.id
        );
        wish::tell_wish(&msg);
    }

    /// Calls the button's command.
    pub fn invoke(&self) {
        let msg = format!("{} invoke", self.id);
        wish::tell_wish(&msg);
    }

    /// Sets the state of the button.
    pub fn state(&self, value: widget::State) {
        widget::configure(&self.id, "state", &value.to_string());
    }
}
