//! Treeview widget
//!
//! A combined tree/list-view widget, for displaying hierarchical data with 
//! multiple values.
//!
//! * also see the Tk [manual](http://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_treeview.htm)

use super::grid;
use super::widget;
use super::wish;

/// Refers to a treeview widget 
#[derive(Clone,Debug,PartialEq)]
pub struct TkTreeview {
    pub id: String,
}

/// Creates an instance of a treeview widget in given parent.
pub fn make_treeview(parent: &impl widget::TkWidget) -> TkTreeview {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::treeview {}", id);
    wish::tell_wish(&msg);

    TkTreeview {
        id,
    }
}

impl widget::TkWidget for TkTreeview {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl grid::TkGridLayout for TkTreeview {
}
