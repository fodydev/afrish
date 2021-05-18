//! Toplevel widgets

use super::grid;
use super::widgets;
use super::wish;

/// Refers to a top-level widget (window)
#[derive(Clone)]
pub struct TkTopLevel {
    pub id: String,
}

impl widgets::TkWidget for TkTopLevel {
    fn id(&self) -> &str {
        &self.id
    }
}

impl TkTopLevel {
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

    // -- functions specific to TkTopLevel

    // TODO - command must accept instance of 'event'
    pub fn bind (&self, event: &str, command: impl Fn()->() + 'static) {
        let event_name = format!("toplevel-{}", event);
        wish::add_callback0(&event_name, wish::mk_callback0(command));
        let msg = format!("bind {} {} {{ puts clicked-{} ; flush stdout }}", 
                          self.id, event, event_name);
        wish::tell_wish(&msg);
    }

    pub fn title(&self, title: &str) {
        let msg = format!("wm title {} {{{}}}\n", self.id, title);
        wish::tell_wish(&msg);
    }
}
