//! Label-frame widget
//!
//! A container widget for other widgets, like frames but with an optional 
//! text label.
//!
//! * also see the Tk [manual](http://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_labelframe.htm)

use super::grid;
use super::widget;
use super::wish;

/// Refers to a label-frame widget
#[derive(Clone)]
pub struct TkLabelFrame {
    pub id: String,
}

/// Creates an instance of a label-frame widget in given parent.
pub fn make_label_frame(parent: &impl widget::TkWidget) -> TkLabelFrame {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::labelframe {}", id);
    wish::tell_wish(&msg);

    TkLabelFrame {
        id,
    }
}

super::tkwidget!(TkLabelFrame);
super::tklayouts!(TkLabelFrame);

impl TkLabelFrame {
    /// Size of border around frame
    pub fn border_width(&self, width: u32) {
        widget::configure(&self.id, "borderwidth", &width.to_string());
    }

    /// Height of frame, in rows
    pub fn height(&self, height: u32) {
        widget::configure(&self.id, "height", &height.to_string());
    }

    /// Position of frame label around the frame.
    ///
    /// Note: `Anchor::Centre` is ignored.
    ///
    pub fn label_anchor(&self, value: widget::Anchor) {
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
        if value != "center" { // ignore centre
            widget::configure(&self.id, "anchor", value);
        }
    }
    
    /// Padding to place around the frame. Takes 
    /// an array of up to four values, specifying: 
    ///
    /// * \[all]
    /// * [left-right top-bottom]
    /// * [left top-bottom right]
    /// * [left top right bottom]
    pub fn padding(&self, values: &[u32]) {
        widget::padding(&self.id, values);
    }

    /// Style of border around frame
    pub fn relief(&self, value: widget::Relief) {
        widget::relief(&self.id, value);
    }

    /// Sets the state of the button (normal or disabled).
    pub fn state(&self, value: widget::State) {
        widget::state(&self.id, value);
    }

    /// Sets the text label for the widget.
    pub fn text(&self, value: &str) {
        widget::configure(&self.id, "text", value);
    }

    /// Underlines the character at the given index position.
    pub fn underline(&self, index: u32) {
        widget::configure(&self.id, "underline", &index.to_string());
    }

    /// Width of frame, in columns
    pub fn width(&self, width: u32) {
        widget::configure(&self.id, "width", &width.to_string());
    }

}

