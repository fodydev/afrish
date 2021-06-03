//! Plotchart - common functions for all charts.
//!

use std::fmt;

use crate::chart::balloon;
use crate::chart::plaintext;
use crate::canvas;
use crate::font;
use crate::image;
use crate::widget;
use crate::wish;

/// Indicates whether a colour should become brighter or darker.
#[derive(Clone, Debug, PartialEq)]
pub enum Brightness {
    Bright,
    Dark,
}

impl fmt::Display for Brightness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Brightness::Bright => "bright",
            Brightness::Dark => "dark",
        };
        write!(f, "{}", &value)
    }
}

/// Type of line to use in drawing chart ticklines.
#[derive(Clone, Debug, PartialEq)]
pub enum ChartDash {
    Dots1,
    Dots2,
    Dots3,
    Dots4,
    Dots5,
    Lines,
}

impl fmt::Display for ChartDash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            ChartDash::Dots1 => "dots1",
            ChartDash::Dots2 => "dots2",
            ChartDash::Dots3 => "dots3",
            ChartDash::Dots4 => "dots4",
            ChartDash::Dots5 => "dots5",
            ChartDash::Lines => "lines",
        };
        write!(f, "{}", &value)
    }
}

/// Defines direction of text against given point.
#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Direction::North => "north",
            Direction::NorthEast => "north-east",
            Direction::East => "east",
            Direction::SouthEast => "south-east",
            Direction::South => "south",
            Direction::SouthWest => "south-west",
            Direction::West => "west",
            Direction::NorthWest => "north-west",
        };
        write!(f, "{}", &value)
    }
}

/// Direction of colour gradient.
#[derive(Clone, Debug, PartialEq)]
pub enum GradientDirection {
    TopDown,
    BottomUp,
    LeftRight,
    RightLeft,
}

impl fmt::Display for GradientDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            GradientDirection::TopDown => "top-down",
            GradientDirection::BottomUp => "bottom-up",
            GradientDirection::LeftRight => "left-right",
            GradientDirection::RightLeft => "right-left",
        };
        write!(f, "{}", &value)
    }
}

/// Defines type of legend.
#[derive(Clone, Debug, PartialEq)]
pub enum LegendType {
    Line,
    Rectangle,
}

impl fmt::Display for LegendType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            LegendType::Line => "line",
            LegendType::Rectangle => "rectangle",
        };
        write!(f, "{}", &value)
    }
}

/// Region of plot to save.
#[derive(Clone, Debug, PartialEq)]
pub enum PlotRegion {
    /// Saves whole plot
    BBox,
    /// Saves visible part only
    Window,
}

impl fmt::Display for PlotRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            PlotRegion::BBox => "bbox",
            PlotRegion::Window => "window",
        };
        write!(f, "{}", &value)
    }
}

/// Defines position of legend with respect to plot.
#[derive(Clone, Debug, PartialEq)]
pub enum Position {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Position::TopLeft => "top-left",
            Position::TopRight => "top-right",
            Position::BottomLeft => "bottom-left",
            Position::BottomRight => "bottom-right",
        };
        write!(f, "{}", &value)
    }
}

/// Functions common to (_almost_) all chart types.
pub trait TkPlotchart {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str;

    /// Sets the background colour for the axes.
    fn background_colour_axes(&self, colour: &str) {
        let msg = format!("global {}; ${} background axes {}",
                          self.id(), self.id(), colour);
        wish::tell_wish(&msg);
    }

    /// Sets a background gradient.
    fn background_colour_gradient(&self, colour: &str, 
                                  direction: GradientDirection, 
                                  brightness: Brightness) {
        let msg = format!("global {}; ${} background gradient {} {} {}",
                          self.id(), self.id(), colour, direction, brightness);
        wish::tell_wish(&msg);
    }

    /// Sets a background image.
    fn background_colour_image(&self, image: &image::TkImage) {
        let msg = format!("global {}; ${} background image {}",
                          self.id(), self.id(), &image.id);
        wish::tell_wish(&msg);
    }

    /// Sets the background colour for the plot area.
    fn background_colour_plot(&self, colour: &str) {
        let msg = format!("global {}; ${} background plot {}",
                          self.id(), self.id(), colour);
        wish::tell_wish(&msg);
    }

    /// Starts definition of balloon text (does not work for 3D plots). 
    ///
    /// Add optional configuration options and finish with `add`.
    ///
    fn balloon(&self, x: f32, y: f32, text: &str) -> balloon::TkChartBalloon {
        balloon::TkChartBalloon::new(self.id(), x, y, text)
    }

    /// Draws a horizontal light-grey band.
    fn draw_x_band(&self, y_min: f32, y_max: f32) {
        let msg = format!("global {}; ${} xband {} {}",
                          self.id(), self.id(), y_min, y_max);
        wish::tell_wish(&msg);
    }

    /// Draws a vertical light-grey band.
    fn draw_y_band(&self, y_min: f32, y_max: f32) {
        let msg = format!("global {}; ${} yband {} {}",
                          self.id(), self.id(), y_min, y_max);
        wish::tell_wish(&msg);
    }

    /// Erases this plot and all associated resources.
    fn erase(&self) {
        let msg = format!("::Plotchart::eraseplot ${}", self.id());
        wish::tell_wish(&msg);
    }

    /// Adds a line to legend for given data series.
    fn legend(&self, series: &str, text: &str, spacing: u32) {
        let msg = format!("global {}; ${} legend {} {{{}}} {}",
                          self.id(), self.id(), 
                          series, text, spacing);
        wish::tell_wish(&msg);
    }

    /// Sets background colour for legend.
    fn legend_background(&self, colour: &str) {
        let msg = format!("global {}; ${} legendconfig -background {}",
                              self.id(), self.id(), colour);
        wish::tell_wish(&msg);
    }

    /// Sets border colour for legend.
    fn legend_border(&self, colour: &str) {
        let msg = format!("global {}; ${} legendconfig -border {}",
                              self.id(), self.id(), colour);
        wish::tell_wish(&msg);
    }

    /// Sets canvas on which to draw legend.
    fn legend_canvas(&self, canvas: &canvas::TkCanvas) {
        let msg = format!("global {}; ${} legendconfig -canvas {}",
                              self.id(), self.id(), &canvas.id);
        wish::tell_wish(&msg);
    }

    /// Sets font with which to draw legend.
    fn legend_font(&self, font: &font::TkFont) {
        let msg = format!("global {}; ${} legendconfig -font {{{}}}",
                              self.id(), self.id(), font);
        wish::tell_wish(&msg);
    }

    /// Position of legend on display.
    fn legend_position(&self, value: Position) {
        let msg = format!("global {}; ${} legendconfig -position {}",
                              self.id(), self.id(), value);
        wish::tell_wish(&msg);
    }

    /// Removes legend entry for given series.
    fn legend_remove(&self, series: &str) {
        let msg = format!("global {}; ${} removefromlegend {{{}}}",
                          self.id(), self.id(), series);
        wish::tell_wish(&msg);
    }

    /// Type of legend to display - series identified by line or colour rectangle.
    fn legend_type(&mut self, value: LegendType) {
        let msg = format!("global {}; ${} legendconfig -legendtype {}",
                              self.id(), self.id(), value);
        wish::tell_wish(&msg);
    }

    /// Starts definition of plain text (does not work for 3D plots). 
    ///
    /// Add optional configuration options and finish with `add`.
    ///
    fn plaintext(&self, x: f32, y: f32, text: &str) -> plaintext::TkChartPlaintext {
        plaintext::TkChartPlaintext::new(self.id(), x, y, text)
    }

    /// Saves chart to a file in postscript format.
    fn save(&self, filename: &str, plotregion: PlotRegion) {
        let msg = format!("global {}; ${} saveplot {{{}}} -plotregion {}",
                          self.id(), self.id(), filename, plotregion);
        wish::tell_wish(&msg);
    }

    /// Sets subtitle of chart.
    fn subtitle(&self, text: &str) {
        let msg = format!(
            "global {}; ${} subtitle {{{}}}",
            self.id(),
            self.id(),
            text,
            );
        wish::tell_wish(&msg);
    }

    /// Sets title of chart.
    fn title(&self, text: &str, placement: widget::Justify) {
        let msg = format!(
            "global {}; ${} title {{{}}} {}",
            self.id(),
            self.id(),
            text,
            placement
            );
        wish::tell_wish(&msg);
    }

    /// Sets subtitle of the (vertical) y-axis, and displays vertically 
    /// along axis.
    fn v_subtitle(&self, text: &str) {
        let msg = format!(
            "global {}; ${} vsubtext {{{}}}",
            self.id(),
            self.id(),
            text,
            );
        wish::tell_wish(&msg);
    }

    /// Sets title of the (vertical) y-axis, and displays vertically 
    /// along axis.
    fn v_title(&self, text: &str) {
        let msg = format!(
            "global {}; ${} vtext {{{}}}",
            self.id(),
            self.id(),
            text,
            );
        wish::tell_wish(&msg);
    }

    /// Sets tcl format string for numbers, see Tk
    /// [manual](https://www.tcl.tk/man/tcl8.5/TclCmd/format.htm)
    fn x_format(&self, format: &str) {
        let msg = format!("global {}; ${} xconfig -format {{{}}}",
                          self.id(), self.id(), format);
        wish::tell_wish(&msg);
    }

    /// Sets space in pixels between label and tickmark.
    fn x_label_offset(&self, value: f32) {
        let msg = format!("global {}; ${} xconfig -labeloffset {}",
                          self.id(), self.id(), value);
        wish::tell_wish(&msg);
    }

    /// Sets number of minor tick marks.
    fn x_minor_ticks(&self, value: u32) {
        let msg = format!("global {}; ${} xconfig -minorticks {}",
                          self.id(), self.id(), value);
        wish::tell_wish(&msg);
    }

    /// Changes x-axis definition to (min, max, step).
    fn x_scale(&self, (min, max, step): (f32, f32, f32)) {
        let msg = format!("global {}; ${} xconfig -scale {{{} {} {} }}",
                          self.id(), self.id(), min, max, step);
        wish::tell_wish(&msg);
    }

    /// Sets subtitle of the (horizontal) x-axis.
    fn x_subtitle(&self, text: &str) {
        let msg = format!(
            "global {}; ${} xsubtext {{{}}}",
            self.id(),
            self.id(),
            text,
            );
        wish::tell_wish(&msg);
    }

    /// Turns on display of vertical ticklines at each tick location.
    fn x_tick_lines(&self, colour: &str, dash: ChartDash) {
        let msg = format!("global {}; ${} xticklines {} {}",
                          self.id(), self.id(), colour, dash);
        wish::tell_wish(&msg);
    }

    /// Sets length in pixels of tick lines.
    fn x_tick_length(&self, value: u32) {
        let msg = format!("global {}; ${} xconfig -ticklines {}",
                          self.id(), self.id(), value);
        wish::tell_wish(&msg);
    }

    /// Sets title of the (horizontal) x-axis.
    fn x_title(&self, text: &str) {
        let msg = format!(
            "global {}; ${} xtext {{{}}}",
            self.id(),
            self.id(),
            text,
            );
        wish::tell_wish(&msg);
    }

    /// Sets tcl format string for numbers, see Tk
    /// [manual](https://www.tcl.tk/man/tcl8.5/TclCmd/format.htm)
    fn y_format(&self, format: &str) {
        let msg = format!("global {}; ${} yconfig -format {{{}}}",
                          self.id(), self.id(), format);
        wish::tell_wish(&msg);
    }

    /// Sets space in pixels between label and tickmark.
    fn y_label_offset(&self, value: f32) {
        let msg = format!("global {}; ${} yconfig -labeloffset {}",
                          self.id(), self.id(), value);
        wish::tell_wish(&msg);
    }

    /// Sets number of minor tick marks.
    fn y_minor_ticks(&self, value: u32) {
        let msg = format!("global {}; ${} yconfig -minorticks {}",
                          self.id(), self.id(), value);
        wish::tell_wish(&msg);
    }

    /// Changes y-axis definition to (min, max, step).
    fn y_scale(&self, (min, max, step): (f32, f32, f32)) {
        let msg = format!("global {}; ${} yconfig -scale {{{} {} {} }}",
                          self.id(), self.id(), min, max, step);
        wish::tell_wish(&msg);
    }

    /// Sets subtitle of the (vertical) y-axis.
    fn y_subtitle(&self, text: &str) {
        let msg = format!(
            "global {}; ${} ysubtext {{{}}}",
            self.id(),
            self.id(),
            text,
            );
        wish::tell_wish(&msg);
    }

    /// Turns on display of vertical ticklines at each tick location.
    fn y_tick_lines(&self, colour: &str, dash: ChartDash) {
        let msg = format!("global {}; ${} yticklines {} {}",
                          self.id(), self.id(), colour, dash);
        wish::tell_wish(&msg);
    }

    /// Sets length in pixels of tick lines.
    fn y_tick_length(&self, value: u32) {
        let msg = format!("global {}; ${} yconfig -ticklines {}",
                          self.id(), self.id(), value);
        wish::tell_wish(&msg);
    }

    /// Sets title of the (vertical) y-axis.
    fn y_title(&self, text: &str) {
        let msg = format!(
            "global {}; ${} ytext {{{}}}",
            self.id(),
            self.id(),
            text,
            );
        wish::tell_wish(&msg);
    }

}
