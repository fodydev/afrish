//! Grid Layout

use super::wish;

#[derive(Clone)]
pub struct GridLayout {
    id: String,
    column: Option<u32>,
    column_span: Option<u32>,
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

    pub fn column_span (&mut self, span: u32) -> &mut Self {
        self.column_span = Some(span);
        self
    }

    pub fn padx (&mut self, pad: u32) -> &mut Self {
        self.padx = Some(pad);
        self
    }

    pub fn pady (&mut self, pad: u32) -> &mut Self {
        self.pady = Some(pad);
        self
    }

    /// Specifies the (0-indexed) row in which to place this widget.
    pub fn row (&mut self, row: u32) -> &mut Self {
        self.row = Some(row);
        self
    }

    pub fn row_span (&mut self, span: u32) -> &mut Self {
        self.row_span = Some(span);
        self
    }

    pub fn sticky (&mut self, sticky: &str) -> &mut Self {
        self.sticky = Some(String::from(sticky));
        self
    }

    /// Called last to finally create the layout with the parameter values 
    /// set up by the builder.
    pub fn layout (&mut self) {
        let mut msg = format!("grid {} ", self.id);
        if let Some(column) = self.column {
            msg.push_str(&format!("-column {} ", column));
        }
        if let Some(span) = self.column_span {
            msg.push_str(&format!("-columnspan {} ", span));
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

