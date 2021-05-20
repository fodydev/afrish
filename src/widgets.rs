use super::wish;

pub trait TkWidget {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str;
}

// macro to write the common widget functions for given struct

#[macro_export]
/// Expands to a set of common functions used in all widgets.
///
/// The first three provide direct access to the equivalent tk functions:
///
/// * cget
/// * configure
/// * winfo
///
/// These functions handle layouts:
///
/// * grid
/// * grid_configure
/// * grid_configure_column
/// * grid_configure_row
///
/// Miscellaneous: 
///
/// * focus
///
macro_rules! tkwidget {
    ($widget:ident) => {
        impl widgets::TkWidget for $widget {
            fn id(&self) -> &str {
                &self.id
            }
        }

        impl $widget {
            /// Retrieve the value of a configuration option
            /// as a string. 
            ///
            /// * `option` - the option to read
            ///
            pub fn cget(&self, option: &str) -> String {
                let msg = format!("{} cget {}", self.id, option);
                wish::eval_wish(&msg)
            }

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

            /// winfo retrieves information about widget.
            ///
            pub fn winfo(&self, option: &str) -> String {
                let msg = format!("winfo {} {}", option, self.id);
                wish::eval_wish(&msg)
            }

            // -- TODO should be here, or more specific?

            /// Makes this widget the focus window (e.g. for key presses)
            pub fn focus(&self) {
                let msg = format!("focus {}", self.id);
                wish::tell_wish(&msg);
            }

            // --- grid layout functions

            /// Creates a GridLayout instance for placing this widget within its parent
            pub fn grid(&self) -> grid::GridLayout {
                grid::GridLayout::new(&self.id)
            }

            /// Sets properties for widget layout
            pub fn grid_configure(&self, option: &str, value: &str) {
                let msg = format!("grid configure {} -{} {{{}}}", self.id, option, value);
                wish::tell_wish(&msg);
            }

            /// Sets property for a given column
            pub fn grid_configure_column(&self, index: u32, option: &str, value: &str) {
                let msg = format!("grid columnconfigure {} {} -{} {{{}}}", self.id, index, option, value);
                wish::tell_wish(&msg);
            }

            /// Sets property for a given row
            pub fn grid_configure_row(&self, index: u32, option: &str, value: &str) {
                let msg = format!("grid rowconfigure {} {} -{} {{{}}}", self.id, index, option, value);
                wish::tell_wish(&msg);
            }

            /// Removes this widget from layout
            pub fn grid_forget(&self) {
                let msg = format!("grid forget {}", self.id);
                wish::tell_wish(&msg);
            }

            // -- winfo functions

            /// Returns the widget x position in pixels, within its parent.
            pub fn position_x(&self) -> u32 {
                let msg = format!("winfo x {}", self.id);
                let result = wish::eval_wish(&msg);
                if let Ok(value) = result.parse::<u32>() {
                    value
                } else {
                    0
                }
            }

            /// Returns the widget y position in pixels, within its parent.
            pub fn position_y(&self) -> u32 {
                let msg = format!("winfo y {}", self.id);
                let result = wish::eval_wish(&msg);
                if let Ok(value) = result.parse::<u32>() {
                    value
                } else {
                    0
                }
            }

            /// Returns the widget height in pixels.
            pub fn widget_height(&self) -> u32 {
                let msg = format!("winfo height {}", self.id);
                let result = wish::eval_wish(&msg);
                if let Ok(value) = result.parse::<u32>() {
                    value
                } else {
                    0
                }
            }

            /// Returns the widget width in pixels.
            pub fn widget_width(&self) -> u32 {
                let msg = format!("winfo width {}", self.id);
                let result = wish::eval_wish(&msg);
                if let Ok(value) = result.parse::<u32>() {
                    value
                } else {
                    0
                }
            }

            /// Returns the position of the mouse on screen of widget as (x,y).
            pub fn mouse_position(&self) -> (i32, i32) {
                (self.mouse_x(), self.mouse_y())
            }

            /// Gives the x position of the mouse on screen of widget.
            pub fn mouse_x(&self) -> i32 {
                let msg = format!("winfo pointerx {}", self.id);
                let result = wish::eval_wish(&msg);
                if let Ok(value) = result.parse::<i32>() {
                    value
                } else {
                    -1
                }
            }

            /// Gives the y position of the mouse on screen of widget.
            pub fn mouse_y(&self) -> i32 {
                let msg = format!("winfo pointery {}", self.id);
                let result = wish::eval_wish(&msg);
                if let Ok(value) = result.parse::<i32>() {
                    value
                } else {
                    -1
                }
            }

            /// Height of screen of widget in pixels.
            pub fn screen_height(&self) -> u32 {
                let msg = format!("winfo screenheight {}", self.id);
                let result = wish::eval_wish(&msg);
                if let Ok(value) = result.parse::<u32>() {
                    value
                } else {
                    0
                }
            }

            /// Height of screen of widget in millimetres.
            pub fn screen_height_mm(&self) -> u32 {
                let msg = format!("winfo screenmmheight {}", self.id);
                let result = wish::eval_wish(&msg);
                if let Ok(value) = result.parse::<u32>() {
                    value
                } else {
                    0
                }
            }

            /// Width of screen of widget in pixels.
            pub fn screen_width(&self) -> u32 {
                let msg = format!("winfo screenwidth {}", self.id);
                let result = wish::eval_wish(&msg);
                if let Ok(value) = result.parse::<u32>() {
                    value
                } else {
                    0
                }
            }

            /// Width of screen of widget in millimetres.
            pub fn screen_width_mm(&self) -> u32 {
                let msg = format!("winfo screenmmwidth {}", self.id);
                let result = wish::eval_wish(&msg);
                if let Ok(value) = result.parse::<u32>() {
                    value
                } else {
                    0
                }
            }
        }
    }
}

// macro to write the common label functions for given struct

#[macro_export]
/// Expands to a set of common functions used in all label, button and 
/// similar widgets.
///
/// * compound
/// * font
/// * foreground
/// * image
/// * padding
/// * text
/// * width
///
/// Also see the Tk [manual](https://tcl.tk/man/tcl/TkCmd/ttk_widget.htm#M6)
///
macro_rules! tklabelfunctions {
    ($widget:ident) => {
        impl $widget {
            /// Specifies how to arrange the text relative to the image.
            pub fn compound(&self, value: widgets::Compound) {
                widgets::compound(&self.id, value);
            }

            /// Specifies the font to use for text.
            pub fn font(&self, definition: &str) {
                widgets::configure(&self.id, "font", definition);
            }

            /// Specifies the foreground (text) colour.
            pub fn foreground(&self, colour: &str) {
                widgets::configure(&self.id, "foreground", colour);
            }

            /// Sets an image to display on the widget.
            pub fn image(&self, image: &image::TkImage) {
                widgets::configure(&self.id, "image", &image.id);
            }

            pub fn padding(&self, pad: &[u32]) {
                let mut values = String::from("");
                for i in pad.iter() {
                   values.push_str(&i.to_string());
                   values.push(' ');
                }
                widgets::configure(&self.id, "padding", &values);
            }

            /// Sets the text label for the widget.
            pub fn text(&self, value: &str) {
                widgets::configure(&self.id, "text", value);
            }

            /// Underlines the character at the given index position.
            pub fn underline(&self, index: u32) {
                widgets::configure(&self.id, "underline", &index.to_string());
            }

            /// Sets the width of the widget, in characters
            pub fn width(&self, value: i32) {
                let msg = format!("{} configure -width {{{}}}", self.id, value);
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

