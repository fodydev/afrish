//! Plotchart
//!
//! Trait of common functions.
//!

use crate::widget;
use crate::wish;

pub trait TkPlotchart {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str;

    /// Sets title of chart
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
}
