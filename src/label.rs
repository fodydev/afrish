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

impl TkLabel {
    // -- common functions to all widgets

    pub fn configure(&self, option: &str, value: &str) {
        widgets::configure(&self.id, option, value);
    }

    pub fn focus(&self) {
        widgets::focus(&self.id);
    }
    
    pub fn grid(&self) -> grid::GridLayout {
        grid::GridLayout::new(&self.id)
    }

    pub fn grid_configure(&self, option: &str, value: &str) {
        widgets::grid_configure(&self.id, option, value);
    }

    pub fn grid_configure_column(&self, index: u32, option: &str, value: &str) {
        widgets::grid_configure_column(&self.id, index, option, value);
    }

    pub fn grid_configure_row(&self, index: u32, option: &str, value: &str) {
        widgets::grid_configure_row(&self.id, index, option, value);
    }

    // -- functions specific to TkLabel

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

