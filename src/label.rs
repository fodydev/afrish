//! Label widgets
//!
//! For displaying some text and/or an image.
//!
//! * also see the Tk [manual](http://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_label.htm)

use super::grid;
use super::image;
use super::widgets;
use super::wish;

/// Refers to a label widget 
#[derive(Clone)]
pub struct TkLabel {
    pub id: String,
}

/// Creates an instance of a label widget in given parent.
pub fn make_label(parent: &impl widgets::TkWidget) -> TkLabel {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::label {}", id);
    wish::tell_wish(&msg);

    TkLabel {
        id,
    }
}

super::tkwidget!(TkLabel);
super::tklabelfunctions!(TkLabel);

impl TkLabel {
    /// Positioning of information with respect to internal margins.
    pub fn anchor(&self, value: widgets::Anchor) {
        let value = match value {
            widgets::Anchor::N => "n",
            widgets::Anchor::NE => "ne",
            widgets::Anchor::E => "e",
            widgets::Anchor::SE => "se",
            widgets::Anchor::S => "s",
            widgets::Anchor::SW => "sw",
            widgets::Anchor::W => "w",
            widgets::Anchor::NW => "nw",
            widgets::Anchor::Center | widgets::Anchor::Centre => "center",
        };
        widgets::configure(&self.id, "anchor", value);
    }

    /// Specifies the background colour.
    pub fn background(&self, colour: &str) {
        widgets::configure(&self.id, "background", colour);
    }

    /// Alignment of text within widget
    pub fn justify(&self, value: widgets::Justify) {
        widgets::justify(&self.id, value);
    }

    /// Style of border around label.
    pub fn relief(&self, value: widgets::Relief) {
        widgets::relief(&self.id, value);
    }

    /// Sets the maximum line length, in pixels. 
    /// When set, displayed text is separated into lines, not exceeding 
    /// the given length.
    pub fn wrap_length(&self, length: u32) {
        widgets::configure(&self.id, "wraplength", &length.to_string());
    }
}

