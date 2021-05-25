//! Canvas widget
//!
//! For drawing interactive graphics.
//!
//! * also see the Tk [manual](http://www.tcl-lang.org/man/tcl8.6/TkCmd/canvas.htm)

use super::grid;
use super::widget;
use super::wish;

/// Refers to a canvas widget
#[derive(Clone)]
pub struct TkCanvas {
    pub id: String,
}

/// Creates an instance of a canvas widget in given parent.
pub fn make_canvas(parent: &impl widget::TkWidget) -> TkCanvas {
    let id = wish::next_wid(parent.id());
    let msg = format!("canvas {}", id);
    wish::tell_wish(&msg);

    TkCanvas {
        id,
    }
}

impl widget::TkWidget for TkCanvas {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}
impl grid::TkGridLayout for TkCanvas {
}


