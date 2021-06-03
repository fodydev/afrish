//! Label widgets - displays text/image.
//!
//! * also see the Tk [manual](https://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_label.htm)

use super::grid;
use super::widget;
use super::wish;

/// Refers to a label widget
#[derive(Clone, Debug, PartialEq)]
pub struct TkLabel {
    pub id: String,
}

/// Creates an instance of a label widget in given parent.
pub fn make_label(parent: &impl widget::TkWidget) -> TkLabel {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::label {}", id);
    wish::tell_wish(&msg);

    TkLabel { id }
}

impl widget::TkWidget for TkLabel {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}
impl grid::TkGridLayout for TkLabel {}
impl widget::TkLabelOptions for TkLabel {}

impl TkLabel {
    /// Positioning of information with respect to internal margins.
    pub fn anchor(&self, value: widget::Anchor) {
        let value = match value {
            widget::Anchor::N => "n",
            widget::Anchor::NE => "ne",
            widget::Anchor::E => "e",
            widget::Anchor::SE => "se",
            widget::Anchor::S => "s",
            widget::Anchor::SW => "sw",
            widget::Anchor::W => "w",
            widget::Anchor::NW => "nw",
            widget::Anchor::Center | widget::Anchor::Centre => "center",
        };
        widget::configure(&self.id, "anchor", value);
    }

    /// Specifies the background colour.
    ///
    /// Colours are specified as a string, by either:
    ///
    /// * `name` - using one of the values in the tk [colours](https://tcl.tk/man/tcl8.6/TkCmd/colors.htm) list
    /// * `rgb` - as a 6-digit hexadecimal value in form "#RRGGBB"
    pub fn background(&self, colour: &str) {
        widget::configure(&self.id, "background", colour);
    }

    /// Alignment of text within widget
    pub fn justify(&self, value: widget::Justify) {
        widget::configure(&self.id, "justify", &value.to_string());
    }

    /// Style of border around label.
    pub fn relief(&self, value: widget::Relief) {
        widget::configure(&self.id, "relief", &value.to_string());
    }

    /// Sets the maximum line length, in pixels.
    /// When set, displayed text is separated into lines, not exceeding
    /// the given length.
    pub fn wrap_length(&self, length: u32) {
        widget::configure(&self.id, "wraplength", &length.to_string());
    }
}
