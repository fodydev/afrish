//! XY Plots - a set of charts for displaying 2D numeric data.
//!

use crate::chart::plotchart;
use crate::canvas;
use crate::wish;

/// Refers to an xy_plot
#[derive(Clone, Debug, PartialEq)]
pub struct TkXYPlot {
    pub id: String,
}

/// Intermediate definition for an xy_plot
#[derive(Clone, Debug, PartialEq)]
pub struct TkXYDefinition {
    canvas_id: String,
    tkname: String, // used in creating the plot
    x_axis: (f64, f64, f64),
    y_axis: (f64, f64, f64),
    x_labels: Option<String>,
    y_labels: Option<String>,
    time_format: Option<String>,
    gmt: Option<bool>,
    axes_at_zero: Option<bool>,
    isometric: Option<bool>,
}

/// Methods to set options for XY-plot - call 'plot' method at 
/// end to finally create the chart.
impl TkXYDefinition {
    /// Adds custom labels for the x-axis.
    pub fn x_labels(&mut self, labels: &[&str]) -> &mut Self {
        let mut labels_str = String::new();
        for label in labels {
            labels_str.push('{');
            labels_str.push_str(label);
            labels_str.push('}');
            labels_str.push(' ');
        }

        self.x_labels = Some(labels_str);
        self
    }

    /// Adds custom labels for the y-axis.
    pub fn y_labels(&mut self, labels: &[&str]) -> &mut Self {
        let mut labels_str = String::new();
        for label in labels {
            labels_str.push('{');
            labels_str.push_str(label);
            labels_str.push('}');
            labels_str.push(' ');
        }

        self.y_labels = Some(labels_str);
        self
    }

    /// Adds string containing tk time format definition.
    pub fn time_format(&mut self, format: &str) -> &mut Self {
        self.time_format = Some(String::from(format));
        self
    }

    /// Sets value for GMT setting (whether to show day-light saving time).
    pub fn gmt(&mut self, value: bool) -> &mut Self {
        self.gmt = Some(value);
        self
    }

    /// Set to true to show axes at (0.0, 0.0) rather than lower-left corner.
    pub fn axes_at_zero(&mut self, value: bool) -> &mut Self {
        self.axes_at_zero = Some(value);
        self
    }

    /// Set to true to draw isometrically, i.e. x/y axes have equal physical size.
    pub fn isometric(&mut self, value: bool) -> &mut Self {
        self.isometric = Some(value);
        self
    }

    /// Completes the definition of an XY-plot and creates the chart.
    pub fn plot(&self) -> TkXYPlot {
        let id = wish::next_wid(&self.canvas_id);
        let mut msg = format!("global {}; set {} [::Plotchart::create{} {} ",
                              id, id, &self.tkname, &self.canvas_id);

        if let Some(labels) = &self.x_labels {
            msg.push_str(&format!("-xlabels {{{}}} ", labels));
        }
        if let Some(labels) = &self.y_labels {
            msg.push_str(&format!("-ylabels {{{}}} ", labels));
        }
        if let Some(value) = &self.time_format {
            msg.push_str(&format!("-timeformat {{{}}} ", value));
        }
        if let Some(value) = self.gmt {
            msg.push_str(&format!("-gmt {} ", if value { "1" } else { "0" }));
        }
        if let Some(value) = self.axes_at_zero {
            msg.push_str(&format!("-axesatzero {} ", if value { "1" } else { "0" }));
        }
        if let Some(value) = self.isometric {
            msg.push_str(&format!("-isometric {} ", if value { "1" } else { "0" }));
        }

        msg.push_str("]");
        wish::tell_wish(&msg);

        TkXYPlot { id }
    }
}

/// Creates an xy_plot.
///
/// Constructor creates an instance of an XY plot definition in given canvas.
///
/// Options must be added and then 'plot' called to finally
/// create the chart.
pub fn make_x_y(
    canvas: &canvas::TkCanvas,
    x_axis: (f64, f64, f64),
    y_axis: (f64, f64, f64)
) -> TkXYDefinition {

    TkXYDefinition { 
        canvas_id: String::from(&canvas.id), 
        tkname: String::from("XYPlot"), 
        x_axis, 
        y_axis,
        x_labels: None,
        y_labels: None,
        time_format: None,
        gmt: None,
        axes_at_zero: None,
        isometric: None,
    }
}

/// Creates an xy_plot where the x axis uses log values.
///
/// Cronstructor ceates an instance of an XY plot definition in given canvas.
///
/// Options must be added and then 'plot' called to finally
/// create the chart.
pub fn make_logx_y(
    canvas: &canvas::TkCanvas,
    x_axis: (f64, f64, f64),
    y_axis: (f64, f64, f64)
) -> TkXYDefinition {

    TkXYDefinition { 
        canvas_id: String::from(&canvas.id), 
        tkname: String::from("LogXYPlot"), 
        x_axis, 
        y_axis,
        x_labels: None,
        y_labels: None,
        time_format: None,
        gmt: None,
        axes_at_zero: None,
        isometric: None,
    }
}

/// Creates an xy_plot where the y axis uses log values.
///
/// Cronstructor ceates an instance of an XY plot definition in given canvas.
///
/// Options must be added and then 'plot' called to finally
/// create the chart.
pub fn make_x_logy(
    canvas: &canvas::TkCanvas,
    x_axis: (f64, f64, f64),
    y_axis: (f64, f64, f64)
) -> TkXYDefinition {

    TkXYDefinition { 
        canvas_id: String::from(&canvas.id), 
        tkname: String::from("XLogYPlot"), 
        x_axis, 
        y_axis,
        x_labels: None,
        y_labels: None,
        time_format: None,
        gmt: None,
        axes_at_zero: None,
        isometric: None,
    }
}

/// Creates an xy_plot where the x and y axes use log values.
///
/// Constructor creates an instance of an XY plot definition in given canvas.
///
/// Options must be added and then 'plot' called to finally
/// create the chart.
pub fn make_logx_logy(
    canvas: &canvas::TkCanvas,
    x_axis: (f64, f64, f64),
    y_axis: (f64, f64, f64)
) -> TkXYDefinition {

    TkXYDefinition { 
        canvas_id: String::from(&canvas.id), 
        tkname: String::from("LogXLogYPlot"), 
        x_axis, 
        y_axis,
        x_labels: None,
        y_labels: None,
        time_format: None,
        gmt: None,
        axes_at_zero: None,
        isometric: None,
    }
}

/// A strip_chart is an xy_plot whose x-axis expands to fit new data.
///
/// Constructor creates an instance of an XY plot definition in given canvas.
///
/// Options must be added and then 'plot' called to finally
/// create the chart.
pub fn make_strip_chart(
    canvas: &canvas::TkCanvas,
    x_axis: (f64, f64, f64),
    y_axis: (f64, f64, f64)
) -> TkXYDefinition {

    TkXYDefinition { 
        canvas_id: String::from(&canvas.id), 
        tkname: String::from("Stripchart"), 
        x_axis, 
        y_axis,
        x_labels: None,
        y_labels: None,
        time_format: None,
        gmt: None,
        axes_at_zero: None,
        isometric: None,
    }
}

impl plotchart::TkPlotchart for TkXYPlot {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl plotchart::TkChartSeries for TkXYPlot {}
impl plotchart::TkChartDots for TkXYPlot {}

impl TkXYPlot {
    /// Draws a box-and-whiskers image on plot, in x direction.
    pub fn box_and_whiskers_x(&self, series: &str, values: &[f64], y: f64) {
        let mut data_str = String::new();
        for value in values {
            data_str.push_str(&format!("{} ", value));
        }

        let msg = format!("global {}; ${} box-and-whiskers {} {{{}}} {}",
                          &self.id, &self.id, series, data_str, y);
        wish::tell_wish(&msg);
    }

    /// Draws a box-and-whiskers image on plot, in y direction.
    pub fn box_and_whiskers_y(&self, series: &str, x: f64, values: &[f64]) {
        let mut data_str = String::new();
        for value in values {
            data_str.push_str(&format!("{} ", value));
        }

        let msg = format!("global {}; ${} box-and-whiskers {} {} {{{}}}",
                          &self.id, &self.id, series, x, data_str);
        wish::tell_wish(&msg);
    }

    /// Draws filled contours for the given values on the grid.
    ///
    /// * Each value is an `(x, y, f)` triple, where `f` is the value at `(x, y)`.
    /// * `classes` is a slice of class levels (leave empty for default).
    pub fn contour_fill(&self, values: &[(f64, f64, f64)], classes: &[f64]) {
        let mut x_data_str = String::new();
        let mut y_data_str = String::new();
        let mut f_data_str = String::new();
        for (x, y, f) in values {
            x_data_str.push_str(&format!("{} ", x));
            y_data_str.push_str(&format!("{} ", y));
            f_data_str.push_str(&format!("{} ", f));
        }
        let mut classes_str = String::new();
        for c in classes {
            classes_str.push_str(&format!("{} ", c));
        }

        let msg = format!("global {}; ${} contourfill {{{}}} {{{}}} {{{}}} {{{}}}",
                          &self.id, &self.id, x_data_str, y_data_str, f_data_str, classes_str);
        wish::tell_wish(&msg);
    }

    /// Draws contour lines for the given values on the grid.
    ///
    /// * Each value is an `(x, y, f)` triple, where `f` is the value at `(x, y)`.
    /// * `classes` is a slice of class levels (leave empty for default).
    pub fn contour_lines(&self, values: &[(f64, f64, f64)], classes: &[f64]) {
        let mut x_data_str = String::new();
        let mut y_data_str = String::new();
        let mut f_data_str = String::new();
        for (x, y, f) in values {
            x_data_str.push_str(&format!("{} ", x));
            y_data_str.push_str(&format!("{} ", y));
            f_data_str.push_str(&format!("{} ", f));
        }
        let mut classes_str = String::new();
        for c in classes {
            classes_str.push_str(&format!("{} ", c));
        }

        let msg = format!("global {}; ${} contourlines {{{}}} {{{}}} {{{}}} {{{}}}",
                          &self.id, &self.id, x_data_str, y_data_str, f_data_str, classes_str);
        wish::tell_wish(&msg);
    }

    /// Draws contour lines for the given values on the grid.
    ///
    /// * Each value is an `(x, y, f)` triple, where `f` is the value at `(x, y)`.
    /// * `classes` is a slice of class levels (leave empty for default).
    pub fn contour_lines_function_values(&self, values: &[(f64, f64, f64)], classes: &[f64]) {
        let mut x_data_str = String::new();
        let mut y_data_str = String::new();
        let mut f_data_str = String::new();
        for (x, y, f) in values {
            x_data_str.push_str(&format!("{} ", x));
            y_data_str.push_str(&format!("{} ", y));
            f_data_str.push_str(&format!("{} ", f));
        }
        let mut classes_str = String::new();
        for c in classes {
            classes_str.push_str(&format!("{} ", c));
        }

        let msg = format!("global {}; ${} contourlinesfunctionvalues {{{}}} {{{}}} {{{}}} {{{}}}",
                          &self.id, &self.id, x_data_str, y_data_str, f_data_str, classes_str);
        wish::tell_wish(&msg);
    }

    /// Draws a dot - value determines symbol/colour.
    pub fn dot(&self, series: &str, (x, y): (f64, f64), value: f64) {
        let msg = format!("global {}; ${} dot {} {} {} {}", 
                          &self.id, &self.id, series, x, y, value);
        wish::tell_wish(&msg);
    }

    /// Plots a single point.
    pub fn plot(&self, series: &str, (x, y): (f64, f64)) {
        let msg = format!("global {}; ${} plot {} {} {}", 
                          &self.id, &self.id, series, x, y);
        wish::tell_wish(&msg);
    }

    /// Plots a list of points.
    ///
    /// * `every` - set to true if a symbol should be drawn for every point
    pub fn plot_list(&self, series: &str, points: &[(f64, f64)], every: bool) {
        let mut x_data_str = String::new();
        let mut y_data_str = String::new();
        for (x, y) in points {
            x_data_str.push_str(&format!("{} ", x));
            y_data_str.push_str(&format!("{} ", y));
        }

        let msg = format!("global {}; ${} plotlist {} {{{}}} {{{}}} {}",
                          &self.id, &self.id, series, 
                          x_data_str, y_data_str,
                          if every { "1" } else { "0" });
        wish::tell_wish(&msg);
    }
}
