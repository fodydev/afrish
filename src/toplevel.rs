//! Toplevel widgets
//!
//! For top-level and popup windows.
//!
//! * also see the Tk [manual](http://www.tcl-lang.org/man/tcl8.6/TkCmd/toplevel.htm)

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
 
    /// Sets the title text on a top-level window.
    pub fn title(&self, title: &str) {
        let msg = format!("wm title {} {{{}}}\n", self.id, title);
        wish::tell_wish(&msg);
    }

}
