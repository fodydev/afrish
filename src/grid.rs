//! Grid Layout
//!
//! A geometry manager for arranging widgets.
//!
//! * also see the Tk [manual](http://www.tcl-lang.org/man/tcl8.6/TkCmd/grid.htm)
//!
//! The grid-layout is used in a "builder" style to layout a single widget:
//!
//! ```
//! widget.grid()
//!   .OPTION(VALUE) // 0 or more
//!   .layout();
//! ```
//!
//! 1. `grid` is called first, to get the GridLayout instance.
//! 2. `layout` must be called last, to perform the layout.
//! 3. zero or more options are added to the GridLayout, to control the position
//!   and layout of the widget.

use super::wish;

/// Refers to the settings for a GridLayout.
#[derive(Clone)]
pub struct GridLayout {
    id: String,
    column: Option<u32>,
    column_span: Option<u32>,
    ipadx: Option<u32>,
    ipady: Option<u32>,
    padx: Option<u32>,
    pady: Option<u32>,
    row: Option<u32>,
    row_span: Option<u32>,
    sticky: Option<String>,
}

impl GridLayout {
    pub(super) fn new(wid: &str) -> GridLayout {
        GridLayout {
            id: String::from(wid),
            column: None,
            column_span: None,
            ipadx: None,
            ipady: None,
            padx: None,
            pady: None,
            row: None,
            row_span: None,
            sticky: None,
        }
    }

    /// Specifies the (0-indexed) column in which to place this widget.
    pub fn column (&mut self, column: u32) -> &mut Self {
        self.column = Some(column);
        self
    }

    /// The number of columns this widget should span.
    pub fn column_span (&mut self, span: u32) -> &mut Self {
        self.column_span = Some(span);
        self
    }

    /// Horizontal padding (inside content border).
    pub fn ipadx (&mut self, pad: u32) -> &mut Self {
        self.ipadx = Some(pad);
        self
    }

    /// Vertical padding (inside content border).
    pub fn ipady (&mut self, pad: u32) -> &mut Self {
        self.ipady = Some(pad);
        self
    }
    
    /// Horizontal padding (outside content border).
    pub fn padx (&mut self, pad: u32) -> &mut Self {
        self.padx = Some(pad);
        self
    }

    /// Vertical padding (outside content border).
    pub fn pady (&mut self, pad: u32) -> &mut Self {
        self.pady = Some(pad);
        self
    }

    /// Specifies the (0-indexed) row in which to place this widget.
    pub fn row (&mut self, row: u32) -> &mut Self {
        self.row = Some(row);
        self
    }

    /// Number of rows this widget should span.
    pub fn row_span (&mut self, span: u32) -> &mut Self {
        self.row_span = Some(span);
        self
    }

    /// When a widget is smaller than its containing space, this 
    /// setting controls how the widget is expanded or positioned
    /// within that space.
    pub fn sticky (&mut self, sticky: &str) -> &mut Self {
        self.sticky = Some(String::from(sticky));
        self
    }

    /// Called last to finally create the layout with the parameter values 
    /// set up by the builder.
    pub fn layout (&self) {
        let mut msg = format!("grid {} ", self.id);
        if let Some(column) = self.column {
            msg.push_str(&format!("-column {} ", column));
        }
        if let Some(span) = self.column_span {
            msg.push_str(&format!("-columnspan {} ", span));
        }
        if let Some(pad) = self.ipadx {
            msg.push_str(&format!("-ipadx {} ", pad));
        }
        if let Some(pad) = self.ipady {
            msg.push_str(&format!("-ipady {} ", pad));
        }
        if let Some(pad) = self.padx {
            msg.push_str(&format!("-padx {} ", pad));
        }
        if let Some(pad) = self.pady {
            msg.push_str(&format!("-pady {} ", pad));
        }
        if let Some(row) = self.row {
            msg.push_str(&format!("-row {} ", row));
        }
        if let Some(span) = self.row_span {
            msg.push_str(&format!("-rowspan {} ", span));
        }
        if let Some(sticky) = &self.sticky {
            msg.push_str(&format!("-sticky {} ", sticky));
        }

        wish::tell_wish(&msg);
    }
}

