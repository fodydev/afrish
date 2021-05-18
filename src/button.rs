//! Button widgets

use super::grid;
use super::image;
use super::widgets;
use super::wish;

/// Refers to a button widget 
#[derive(Clone)]
pub struct TkButton {
    pub id: String,
}

pub fn make_button(parent: &impl widgets::TkWidget) -> TkButton {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::button {}", id);
    wish::tell_wish(&msg);

    TkButton {
        id,
    }
}

impl TkButton {
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

    // -- functions specific to TkButton

    pub fn command (&self, command: impl Fn()->() + 'static) {
        wish::add_callback0(&self.id, wish::mk_callback0(command));
        let msg = format!("{} configure -command {{ puts clicked-{} ; flush stdout }}", self.id, self.id);
        wish::tell_wish(&msg);
    }
    
    pub fn compound(&self, value: widgets::Compound) {
        widgets::compound(&self.id, value);
    }
    
    pub fn image(&self, image: &image::TkImage) {
        widgets::configure(&self.id, "image", &image.id);
    }

    pub fn state(&self, value: widgets::State) {
        widgets::state(&self.id, value);
    }

    pub fn text(&self, value: &str) {
        widgets::configure(&self.id, "text", value);
    }
}

