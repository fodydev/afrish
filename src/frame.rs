//! Frame widget
//!
//! A container widget for other widgets.
//!
//! * also see the Tk [manual](http://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_frame.htm)

use super::grid;
use super::widgets;
use super::wish;

/// Refers to a frame widget
#[derive(Clone)]
pub struct TkFrame {
    pub id: String,
}

/// Creates an instance of a frame widget in given parent.
pub fn make_frame(parent: &impl widgets::TkWidget) -> TkFrame {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::frame {}", id);
    wish::tell_wish(&msg);

    TkFrame {
        id,
    }
}

super::tkwidget!(TkFrame);

impl TkFrame {
    /// Size of border around frame
    pub fn border_width(&self, width: u32) {
        widgets::configure(&self.id, "borderwidth", &width.to_string());
    }

    /// Height of frame, in rows
    pub fn height(&self, height: u32) {
        widgets::configure(&self.id, "height", &height.to_string());
    }

    /// Padding to place around the frame. Takes 
    /// an array of up to four values, specifying: 
    ///
    /// * \[all]
    /// * [left-right top-bottom]
    /// * [left top-bottom right]
    /// * [left top right bottom]
    pub fn padding(&self, values: &[u32]) {
        widgets::padding(&self.id, values);
    }

    /// Style of border around frame
    pub fn relief(&self, value: widgets::Relief) {
        widgets::relief(&self.id, value);
    }

    /// Width of frame, in columns
    pub fn width(&self, width: u32) {
        widgets::configure(&self.id, "width", &width.to_string());
    }

}

