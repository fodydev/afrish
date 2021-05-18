//! Entry widgets

use super::grid;
use super::widgets;
use super::wish;

/// Refers to an entry widget 
#[derive(Clone)]
pub struct TkEntry {
    pub id: String,
}

pub fn make_entry(parent: &impl widgets::TkWidget) -> TkEntry {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::entry {}", id);
    wish::tell_wish(&msg);

    TkEntry {
        id,
    }
}

impl TkEntry {
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

    // -- functions specific to TkEntry

    pub fn width (&self, value: u32) {
        let msg = format!("{} configure -width {{{}}}", self.id, value);
        wish::tell_wish(&msg);
    }
}

