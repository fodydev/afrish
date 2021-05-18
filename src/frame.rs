
use super::grid;
use super::widgets;
use super::wish;

#[derive(Clone)]
pub struct TkFrame {
    pub id: String,
}

pub fn make_frame(parent: &impl widgets::TkWidget) -> TkFrame {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::frame {}", id);
    wish::tell_wish(&msg);

    TkFrame {
        id,
    }
}

impl widgets::TkWidget for TkFrame {
    fn id(&self) -> &str {
        &self.id
    }
}

impl TkFrame {
    // -- common functions to all widgets

    pub fn configure(&self, option: &str, value: &str) {
        widgets::configure(&self.id, option, value);
    }

    pub fn focus(&self) {
        widgets::focus(&self.id);
    }
    
    pub fn grid(&self) -> grid::GridLayout {
        grid::GridLayout::new(&self.id)
    }

    pub fn grid_configure(&self, option: &str, value: &str) {
        widgets::grid_configure(&self.id, option, value);
    }

    pub fn grid_configure_column(&self, index: u32, option: &str, value: &str) {
        widgets::grid_configure_column(&self.id, index, option, value);
    }

    pub fn grid_configure_row(&self, index: u32, option: &str, value: &str) {
        widgets::grid_configure_row(&self.id, index, option, value);
    }

    // -- functions specific to TkFrame

    pub fn border_width(&self, width: u32) {
        widgets::configure(&self.id, "borderwidth", &width.to_string());
    }

    pub fn relief (&self, value: widgets::Relief) {
        let value = match value {
            widgets::Relief::Flat => "flat",
            widgets::Relief::Groove => "groove",
            widgets::Relief::Raised => "raised",
            widgets::Relief::Ridge => "ridge",
            widgets::Relief::Solid => "solid",
            widgets::Relief::Sunken => "sunken",
        };
        widgets::configure(&self.id, "relief", value);
    }

}

