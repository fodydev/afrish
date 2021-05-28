//! Scale widget
//!
//! A scale widget is shown as a slider over a range, used to select a number
//! from a range of values.
//!
//! * also see the Tk [manual](http://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_scale.htm)
//!

use super::grid;
use super::widget;
use super::wish;

/// Refers to a scale widget 
#[derive(Clone,Debug,PartialEq)]
pub struct TkScale {
    pub id: String,
}

/// Creates an instance of a scale widget in given parent,
/// oriented horizontally.
pub fn make_horizontal_scale(parent: &impl widget::TkWidget) -> TkScale {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::scale {} -orient horizontal", id);
    wish::tell_wish(&msg);

    TkScale {
        id,
    }
}

/// Creates an instance of a scale widget in given parent,
/// oriented vertically.
pub fn make_vertical_scale(parent: &impl widget::TkWidget) -> TkScale {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::scale {} -orient vertical", id);
    wish::tell_wish(&msg);

    TkScale {
        id,
    }
}

impl widget::TkWidget for TkScale {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl grid::TkGridLayout for TkScale {
}

impl TkScale {
    /// Sets the function to be called whenever the scale value is changed.
    pub fn command (&self, command: impl Fn(f32)->() + Send + 'static) {
        wish::add_callback1_float(&self.id, wish::mk_callback1_float(command));
        let msg = format!("{} configure -command [list scale_value {}]", 
                          self.id, self.id);
        wish::tell_wish(&msg);
    }

    /// Sets the minimum value for the scale.
    pub fn from(&self, value: f32) {
        widget::configure(&self.id, "from", &value.to_string());
    }

    /// Sets the maximum value for the scale.
    pub fn to(&self, value: f32) {
        widget::configure(&self.id, "to", &value.to_string());
    }

    /// Retrieves the scale's value.
    pub fn value(&self) -> f32 {
        let msg = format!("puts [{} get] ; flush stdout", self.id);
        let result = wish::eval_wish(&msg);
        if let Ok(value) = result.parse::<f32>() {
            value
        } else {
            0.0
        }
    }

    /// Set the scale's value.
    pub fn value_set(&self, value: f32) {
        widget::configure(&self.id, "value", &value.to_string());
    }

    /// Displayed length of scale in pixels.
    pub fn length(&self, value: u32) {
        widget::configure(&self.id, "length", &value.to_string());
    }

    /// Sets the state of the widget (normal or disabled).
    pub fn state(&self, value: widget::State) {
        widget::state(&self.id, value);
    }
}

