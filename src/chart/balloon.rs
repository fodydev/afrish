//! Balloon text - builder functions for creating balloon text.
//!
//! The balloon text is defined in a "builder" style:
//!
//! ```
//! chart.balloon(x, y, "some text")
//!   .OPTION(VALUE) // 0 or more
//!   .add();
//! ```
//!
//! 1. `balloon` is called first, with the (x, y) coordinates and text to show.
//! 2. `add` must be called last, to add the text with the given options.
//! 3. zero or more options are added to the balloon text, to control the 
//!    display and layout of the text.

use crate::font;
use crate::plotchart;
use crate::widget;
use crate::wish;

/// Refers to the settings for a balloon text.
///
/// The methods on this struct set the values of different options in a 
/// builder style: call `add` to finish and place the balloon text.
///
#[derive(Clone, Debug, PartialEq)]
pub struct TkChartBalloon {
    wid: String,
    x: f32,
    y: f32,
    text: String,
    direction: plotchart::Direction,
    font: Option<font::TkFont>,
    justify: Option<widget::Justify>,
    text_colour: Option<String>,
    background: Option<String>,
    outline_colour: Option<String>,
    margin: Option<u32>,
    rim_width: Option<u32>,
    arrow_size: Option<u32>,
}

impl TkChartBalloon {
    pub fn new(wid: &str, x: f32, y: f32, text: &str) -> TkChartBalloon {
        TkChartBalloon {
            wid: String::from(wid),
            x,
            y,
            text: String::from(text),
            direction: plotchart::Direction::North,
            font: None,
            justify: None,
            text_colour: None,
            background: None,
            outline_colour: None,
            margin: None,
            rim_width: None,
            arrow_size: None,
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

    /// Colour of background.
    pub fn background(&mut self, colour: &str) -> &mut Self {
        self.background = Some(String::from(colour));
        self
    }

    /// Colour of outline.
    pub fn outline_colour(&mut self, colour: &str) -> &mut Self {
        self.outline_colour = Some(String::from(colour));
        self
    }

    /// Margin size in pixels, around text.
    pub fn margin(&mut self, value: u32) -> &mut Self {
        self.margin = Some(value);
        self
    }

    /// Width in pixels, of margin.
    pub fn rim_width(&mut self, value: u32) -> &mut Self {
        self.rim_width = Some(value);
        self
    }

    /// Arrow length, in pixels.
    pub fn arrow_size(&mut self, value: u32) -> &mut Self {
        self.arrow_size = Some(value);
        self
    }

    /// Called last to finally create the balloon text with the parameter
    /// values set up by the builder.
    pub fn add(&self) {
        let mut msg = format!("global {}; ${} balloonconfig ",
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
        if let Some(colour) = &self.background {
            msg.push_str(&format!("-background {} ", colour));
        }
        if let Some(colour) = &self.outline_colour {
            msg.push_str(&format!("-outline {} ", colour));
        }
        if let Some(value) = self.margin {
            msg.push_str(&format!("-margin {} ", value));
        }
        if let Some(value) = self.rim_width {
            msg.push_str(&format!("-rimwidget {} ", value));
        }
        if let Some(value) = self.arrow_size {
            msg.push_str(&format!("-arrowsize {} ", value));
        }
        wish::tell_wish(&msg);

        let msg = format!("global {}; ${} balloon {} {} {{{}}} {}",
                          &self.wid, &self.wid, 
                          self.x, self.y,
                          &self.text,
                          self.direction);
        wish::tell_wish(&msg);
    }
}
