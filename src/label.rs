//! Label widgets

use super::grid;
use super::image;
use super::widgets;
use super::wish;

/// Refers to a label widget 
#[derive(Clone)]
pub struct TkLabel {
    pub id: String,
}

pub fn make_label(parent: &impl widgets::TkWidget) -> TkLabel {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::label {}", id);
    wish::tell_wish(&msg);

    TkLabel {
        id,
    }
}

super::tkwidget!(TkLabel);

impl TkLabel {
    pub fn compound(&self, value: widgets::Compound) {
        widgets::compound(&self.id, value);
    }
 
    pub fn font(&self, font: &str) {
        widgets::configure(&self.id, "font", font);
    }

    pub fn image(&self, image: &image::TkImage) {
        widgets::configure(&self.id, "image", &image.id);
    }

    pub fn text(&self, value: &str) {
        widgets::configure(&self.id, "text", value);
    }

    pub fn wrap_length(&self, len: u32) {
        widgets::configure(&self.id, "wraplength", &len.to_string());
    }
}

