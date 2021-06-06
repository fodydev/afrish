//! Polar plot - plots points along a circular axis.
//!

use crate::chart::plotchart;
use crate::canvas;
use crate::widget;
use crate::wish;

/// Refers to a polar plot
#[derive(Clone, Debug, PartialEq)]
pub struct TkPolarPlot {
    pub id: String,
}
/// Intermediate definition for a polar plot
#[derive(Clone, Debug, PartialEq)]
pub struct TkPolarDefinition {
    canvas_id: String,
    radius_data: (f64, f64),
    x_labels: Option<String>,
    y_labels: Option<String>,
    axes_at_zero: Option<bool>,
    isometric: Option<bool>,
}

/// Methods to set options for polar-plot - call 'plot' method at
/// end to finally create the chart.
impl TkPolarDefinition {
    /// Adds custom labels for the x-axis.
    pub fn x_labels(&mut self, labels: &[&str]) -> &mut Self {
        self.x_labels = Some(widget::strings_list(labels));
        self
    }

    /// Adds custom labels for the y-axis.
    pub fn y_labels(&mut self, labels: &[&str]) -> &mut Self {
        self.y_labels = Some(widget::strings_list(labels));
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

    /// Completes the definition of a polar-plot and creates the chart.
    pub fn plot(&self) -> TkPolarPlot {
        let id = wish::next_var();
        let mut msg = format!(
            "global {}; set {} [::Plotchart::createPolarplot {} {{{} {}}} ",
            id, id, &self.canvas_id,
            self.radius_data.0, self.radius_data.1
        );

        if let Some(labels) = &self.x_labels {
            msg.push_str(&format!("-xlabels {{{}}} ", labels));
        }
        if let Some(labels) = &self.y_labels {
            msg.push_str(&format!("-ylabels {{{}}} ", labels));
        }
        if let Some(value) = self.axes_at_zero {
            msg.push_str(&format!("-axesatzero {} ", if value { "1" } else { "0" }));
        }
        if let Some(value) = self.isometric {
            msg.push_str(&format!("-isometric {} ", if value { "1" } else { "0" }));
        }

        msg.push_str("]");
        wish::tell_wish(&msg);

        TkPolarPlot { id }
    }
}

/// Creates an instance of a polar plot in given canvas.
///
/// Constructor creates an instance of a polar-plot definition in given canvas.
///
/// Options must be added and then 'plot' called to finally
/// create the chart.
pub fn make_polar(canvas: &canvas::TkCanvas, 
                  radius_data: (f64, f64)) -> TkPolarDefinition {
    TkPolarDefinition {
        canvas_id: String::from(&canvas.id),
        radius_data,
        x_labels: None,
        y_labels: None,
        axes_at_zero: None,
        isometric: None,
    }
}

impl plotchart::TkPlotchart for TkPolarPlot {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl plotchart::TkChartSeries for TkPolarPlot {}
impl plotchart::TkChartDots for TkPolarPlot {}

impl TkPolarPlot {
    /// Draws a dot - value determines symbol/colour.
    pub fn draw_dot(&self, series: &str, (x, y): (f64, f64), value: f64) {
        let msg = format!(
            "global {}; ${} dot {} {} {} {}",
            &self.id, &self.id, series, x, y, value
        );
        wish::tell_wish(&msg);
    }

    /// Draws a dot with a text label.
    ///
    /// See [TkChartDots](plotchart::TkChartDots) for configuration options.
    pub fn draw_labelled_dot(
        &self,
        (x, y): (f64, f64),
        label: &str,
        location: plotchart::Location,
    ) {
        let msg = format!(
            "global {}; ${} labeldot {} {} {} {}",
            &self.id, &self.id, x, y, label, location
        );
        wish::tell_wish(&msg);
    }

    /// Draws a band on chart, parallel to x-axis so it appears as a ring.
    ///
    /// * `series` - name of data series
    /// * `x_coord` - x-coordinate
    /// * `y_min` - minimum y-value
    /// * `y_max` - maximum y-value
    pub fn draw_minmax(&self, series: &str, x_coord: f64, y_min: f64, y_max: f64) {
        let msg = format!(
            "global {}; ${} minmax {} {} {} {}",
            &self.id, &self.id, series, x_coord, y_min, y_max
        );
        wish::tell_wish(&msg);
    }

    /// Draws a closed polygon, using given points.
    pub fn draw_region(&self, series: &str, points: &[(f64, f64)]) {
        let xs: Vec<f64> = points.iter().map(|(x, _)| x).cloned().collect();
        let ys: Vec<f64> = points.iter().map(|(_, y)| y).cloned().collect();

        let msg = format!(
            "global {}; ${} region {} {{{}}} {{{}}}",
            &self.id,
            &self.id,
            series,
            &widget::str_list(&xs),
            &widget::str_list(&ys),
        );
        wish::tell_wish(&msg);
    }

    /// Plots a single point with respect to this axis.
    pub fn plot(&self, series: &str, (x, y): (f64, f64)) {
        let msg = format!("global {}; ${} plot {} {} {}", 
                          &self.id, &self.id, series, x, y);
        wish::tell_wish(&msg);
    }
}

