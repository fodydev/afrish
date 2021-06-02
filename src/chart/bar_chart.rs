//! Bar chart

use crate::canvas;
use crate::chart::plotchart;
use crate::wish;

/// Refers to a bar chart
#[derive(Clone, Debug, PartialEq)]
pub struct TkBarChart {
    pub id: String,
}

/// Creates an instance of a bar chart in given canvas.
pub fn make_bar_chart(
    canvas: &canvas::TkCanvas,
    x_labels: &[&str],
    y_axis: (f32, f32, f32),
    num_series: u32,
) -> TkBarChart {
    let mut labels_str = String::new();
    for label in x_labels {
        labels_str.push('{');
        labels_str.push_str(label);
        labels_str.push('}');
        labels_str.push(' ');
    }

    let id = wish::next_var();
    let msg = format!(
        "global {}; set {} [::Plotchart::createBarchart {} {{{}}} {{ {} {} {} }} {}]",
        id, id, &canvas.id, labels_str, y_axis.0, y_axis.1, y_axis.2, num_series
    );
    wish::tell_wish(&msg);

    TkBarChart { id }
}

impl plotchart::TkPlotchart for TkBarChart {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}
