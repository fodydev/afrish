use super::wish;

pub trait TkWidget {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str;
}

// macro to write the common widget functions for given struct

#[macro_export]
/// Expands to a set of common functions used in all widgets.
///
/// * configure
/// * focus
/// * grid
/// * grid_configure
/// * grid_configure_column
/// * grid_configure_row
///
macro_rules! tkwidget {
    ($widget:ident) => {
        impl widgets::TkWidget for $widget {
            fn id(&self) -> &str {
                &self.id
            }
        }

        impl $widget {
            /// Used to change properties of a widget. 
            /// This function can be used to directly configure
            /// the widget using an option-value string pair: 
            ///
            /// * `option` - the option to change
            /// * `value` - the value to change it to
            ///
            pub fn configure(&self, option: &str, value: &str) {
                widgets::configure(&self.id, option, value);
            }

            /// Makes this widget the focus window (e.g. for key presses)
            pub fn focus(&self) {
                let msg = format!("focus {}", self.id);
                wish::tell_wish(&msg);
            }

            /// Creates a GridLayout instance for placing this widget within its parent
            pub fn grid(&self) -> grid::GridLayout {
                grid::GridLayout::new(&self.id)
            }

            pub fn grid_configure(&self, option: &str, value: &str) {
                let msg = format!("grid configure {} -{} {{{}}}", self.id, option, value);
                wish::tell_wish(&msg);
            }

            pub fn grid_configure_column(&self, index: u32, option: &str, value: &str) {
                let msg = format!("grid columnconfigure {} {} -{} {{{}}}", self.id, index, option, value);
                wish::tell_wish(&msg);
            }

            pub fn grid_configure_row(&self, index: u32, option: &str, value: &str) {
                let msg = format!("grid rowconfigure {} {} -{} {{{}}}", self.id, index, option, value);
                wish::tell_wish(&msg);
            }
        }
    }
}

// --------------------------------------------------------------------------
// Enums to type-check values

pub enum Compound {
    Bottom,
    Center,
    Centre,
    Image,
    Left,
    None,
    Right,
    Text,
    Top,
}

pub enum Relief {
    Flat,
    Groove,
    Raised,
    Ridge,
    Solid,
    Sunken,
}

/// The kinds of activity state for a widget, e.g. if it is currently
/// available to use or disabled.
pub enum State {
    /// Used, e.g., for buttons, to highlight when a mouse pointer is over them.
    Active,
    /// Used to prevent user-interaction with a widget.
    Disabled,
    /// The usual state of a widget, permitting user interactions.
    Normal,
    /// State cannot be changed, for those widgets with editable state.
    Readonly,
}

// --------------------------------------------------------------------------
// Internal functions for within crate use

pub(super) fn compound (wid: &str, value: Compound) {
    let value = match value {
        Compound::Bottom => "bottom",
        Compound::Center | Compound::Centre => "center",
        Compound::Image => "image",
        Compound::Left => "left",
        Compound::None => "none",
        Compound::Right => "right",
        Compound::Text => "text",
        Compound::Top => "top",
    };
    configure(wid, "compound", value);
}

pub(super) fn state (wid: &str, value: State) {
    let value = match value {
        State::Active => "active",
        State::Disabled => "disabled",
        State::Normal => "normal",
        State::Readonly => "readonly",
    };
    configure(wid, "state", value);
}

pub(super) fn configure(wid: &str, option: &str, value: &str) {
    let msg = format!("{} configure -{} {{{}}}", wid, option, value);
    wish::tell_wish(&msg);
}

pub(super) fn screen_height(wid: &str) -> u32 {
    0
}
