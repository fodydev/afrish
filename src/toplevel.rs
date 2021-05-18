//! Toplevel widgets

use super::grid;
use super::widgets;
use super::wish;

/// Refers to a top-level widget (window)
#[derive(Clone)]
pub struct TkTopLevel {
    pub id: String,
}

super::tkwidget!(TkTopLevel);

impl TkTopLevel {
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
