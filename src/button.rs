//! Button widgets

use super::grid;
use super::image;
use super::widgets;
use super::wish;

/// Refers to a button widget 
#[derive(Clone)]
pub struct TkButton {
    pub id: String,
}

/// Creates an instance of a button widget in given parent.
pub fn make_button(parent: &impl widgets::TkWidget) -> TkButton {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::button {}", id);
    wish::tell_wish(&msg);

    TkButton {
        id,
    }
}

super::tkwidget!(TkButton);

impl TkButton {
    /// Sets the function to be called when the button is clicked.
    pub fn command (&self, command: impl Fn()->() + 'static) {
        wish::add_callback0(&self.id, wish::mk_callback0(command));
        let msg = format!("{} configure -command {{ puts clicked-{} ; flush stdout }}", self.id, self.id);
        wish::tell_wish(&msg);
    }
    
    /// For buttons with text and images, specifies how to arrange the text
    /// relative to the image.
    pub fn compound(&self, value: widgets::Compound) {
        widgets::compound(&self.id, value);
    }
    
    /// Sets an image to display on the button.
    pub fn image(&self, image: &image::TkImage) {
        widgets::configure(&self.id, "image", &image.id);
    }

    /// Sets the state of the button (normal or disabled).
    pub fn state(&self, value: widgets::State) {
        widgets::state(&self.id, value);
    }

    /// Sets the text label for the button.
    pub fn text(&self, value: &str) {
        widgets::configure(&self.id, "text", value);
    }
}

