//! Plain text - builder functions for creating plain text.
//!
//! The plain text is defined in a "builder" style:
//!
//! ```
//! chart.plaintext(x, y, "some text")
//!   .OPTION(VALUE) // 0 or more
//!   .add();
//! ```
//!
//! 1. `plaintext` is called first, with the (x, y) coordinates and text to show.
//! 2. `add` must be called last, to add the text with the given options.
//! 3. zero or more options are added to the plain text, to control the 
//!    display and layout of the text.

use crate::font;
use crate::plotchart;
use crate::widget;
use crate::wish;

/// Refers to the settings for a plain text.
///
/// The methods on this struct set the values of different options in a 
/// builder style: call `add` to finish and place the text.
///
#[derive(Clone, Debug, PartialEq)]
pub struct TkChartPlaintext {
    wid: String,
    x: f32,
    y: f32,
    text: String,
    direction: plotchart::Direction,
    font: Option<font::TkFont>,
    justify: Option<widget::Justify>,
    text_colour: Option<String>,
}

impl TkChartPlaintext {
    pub fn new(wid: &str, x: f32, y: f32, text: &str) -> TkChartPlaintext {
        TkChartPlaintext {
            wid: String::from(wid),
            x,
            y,
            text: String::from(text),
            direction: plotchart::Direction::North,
            font: None,
            justify: None,
            text_colour: None,
        }
    }

    /// Font used for text.
    pub fn font(&mut self, font: &font::TkFont) -> &mut Self {
        self.font = Some(font.clone());
        self
    }

    /// Justification of text.
    pub fn justify(&mut self, value: widget::Justify) -> &mut Self {
        self.justify = Some(value);
        self
    }

    /// Colour of text.
    pub fn text_colour(&mut self, colour: &str) -> &mut Self {
        self.text_colour = Some(String::from(colour));
        self
    }

    /// Called last to finally create the plain text with the parameter
    /// values set up by the builder.
    pub fn add(&self) {
        let mut msg = format!("global {}; ${} plaintextconfig ",
                              &self.wid, &self.wid);
        if let Some(font) = &self.font {
            msg.push_str(&format!("-font {{{}}} ", font));
        }
        if let Some(justify) = &self.justify {
            msg.push_str(&format!("-justify {} ", justify));
        }
        if let Some(colour) = &self.text_colour {
            msg.push_str(&format!("-textcolour {} ", colour));
        }
        wish::tell_wish(&msg);

        let msg = format!("global {}; ${} plaintext {} {} {{{}}} {}",
                          &self.wid, &self.wid, 
                          self.x, self.y,
                          &self.text,
                          self.direction);
        wish::tell_wish(&msg);
    }
}
