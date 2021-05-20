
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

super::tkwidget!(TkFrame);

impl TkFrame {
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

    pub fn height(&self, height: u32) {
        widgets::configure(&self.id, "height", &height.to_string());
    }

    pub fn width(&self, width: u32) {
        widgets::configure(&self.id, "width", &width.to_string());
    }

}

