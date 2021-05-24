//! Separator widget.
//!
//! A simple widget, that displays a separating line in its parent.
//! The separator is either oriented vertically or horizontally.

use super::grid;
use super::widget;
use super::wish;

/// Refers to a separator widget
#[derive(Clone)]
pub struct TkSeparator {
    pub id: String,
}

/// Creates an instance of a horizontal separator in given parent.
pub fn make_horizontal_separator(parent: &impl widget::TkWidget) -> TkSeparator {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::separator {} -orient horizontal", id);
    wish::tell_wish(&msg);

    TkSeparator {
        id,
    }
}

/// Creates an instance of a vertical separator in given parent.
pub fn make_vertical_separator(parent: &impl widget::TkWidget) -> TkSeparator {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::separator {} -orient vertical", id);
    wish::tell_wish(&msg);

    TkSeparator {
        id,
    }
}

super::tkwidget!(TkSeparator);
super::tklayouts!(TkSeparator);

