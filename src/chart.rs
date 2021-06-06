//! Plotchart wrapper - draws different kinds of charts on a canvas.
//!
//! For more complete descriptions, see the original plotchart
//! [documentation](https://core.tcl-lang.org/tklib/doc/trunk/embedded/www/tklib/files/modules/plotchart/plotchart.html).

pub mod bar_chart;
pub use bar_chart::*;

pub mod box_plot;
pub use box_plot::*;

pub mod isometric_plot;
pub use isometric_plot::*;

pub mod pie_chart;
pub use pie_chart::*;

pub mod plotchart;
pub use plotchart::*;

pub mod polar_plot;
pub use polar_plot::*;

pub mod radial_chart;
pub use radial_chart::*;

pub mod right_axis;
pub use right_axis::*;

pub mod tx_plot;
pub use tx_plot::*;

pub mod xy_plots;
pub use xy_plots::*;

