//! Text widget - TO COMPLETE
//!
//! A widget for displaying text.
//!
//! * also see the Tk [manual](http://www.tcl-lang.org/man/tcl8.6/TkCmd/text.htm)

use super::grid;
use super::widget;
use super::wish;

/// Refers to a text widget 
#[derive(Clone,Debug,PartialEq)]
pub struct TkText {
    pub id: String,
}

/// Creates an instance of a text widget in given parent.
pub fn make_text(parent: &impl widget::TkWidget) -> TkText {
    let id = wish::next_wid(parent.id());
    let msg = format!("text {}", id);
    wish::tell_wish(&msg);

    TkText {
        id,
    }
}

impl widget::TkWidget for TkText {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl grid::TkGridLayout for TkText {
}

impl TkText {
    /// Add to end of text.
    pub fn append(&self, text: &str) {
        let msg = format!("{} insert end {{{}}}", &self.id, text);
        wish::tell_wish(&msg);
    }

    /// Delete a range of text.
    pub fn delete(&self, 
                  (from_line, from_character): (u32, u32), 
                  (to_line, to_character): (u32, u32)) {
        let msg = format!("{} delete {}.{} {}.{}",
                          &self.id, from_line, from_character, 
                          to_line, to_character);
        wish::tell_wish(&msg);
    }

    /// Specifies the font to use for text.
    pub fn font(&self, definition: &str) {
        widget::configure(&self.id, "font", definition);
    }
    
    /// Get a range of text.
    pub fn get(&self, 
               (from_line, from_character): (u32, u32), 
               (to_line, to_character): (u32, u32)) -> String {
        let msg = format!("puts [{} get {}.{} {}.{}] ; flush stdout",
                          &self.id, from_line, from_character, 
                          to_line, to_character);
        wish::eval_wish(&msg)
    }

    /// Get a range of text from a position to end.
    pub fn get_to_end(&self,
               (from_line, from_character): (u32, u32)) -> String {
        let msg = format!("puts [{} get {}.{} end] ; flush stdout",
                          &self.id, from_line, from_character);
        wish::eval_wish(&msg)
    }

    /// Height of text, in rows
    pub fn height(&self, height: u32) {
        widget::configure(&self.id, "height", &height.to_string());
    }

    /// Insert at given (line, character) position of text.
    pub fn insert(&self, (line, character): (u32, u32), text: &str) {
        let msg = format!("{} insert {}.{} {{{}}}", 
                          self.id, line, character, text);
        wish::tell_wish(&msg);
    }

    /// Sets the state of the widget (`normal` or `disabled` only).
    pub fn state(&self, value: widget::State) {
        widget::configure(&self.id, "state", &value.to_string());
    }

    /// Width of text, in columns
    pub fn width(&self, width: u32) {
        widget::configure(&self.id, "width", &width.to_string());
    }

    /// How wrapping should be performed of long lines.
    pub fn wrap(&self, value: widget::Wrapping) {
        widget::configure(&self.id, "wrap", &value.to_string());
    }
}

