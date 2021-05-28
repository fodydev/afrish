//! Toplevel widgets
//!
//! For top-level and popup windows.
//!
//! * also see the Tk [manual](http://www.tcl-lang.org/man/tcl8.6/TkCmd/toplevel.htm)

use super::menu;
use super::widget;
use super::wish;

/// Refers to a top-level widget (window)
#[derive(Clone,Debug,PartialEq)]
pub struct TkTopLevel {
    pub id: String,
}

/// Creates an instance of a toplevel widget with given parent.
pub fn make_toplevel(parent: &impl widget::TkWidget) -> TkTopLevel {
    let id = wish::next_wid(parent.id());
    let msg = format!("toplevel {}", id);
    wish::tell_wish(&msg);

    TkTopLevel {
        id,
    }
}

impl widget::TkWidget for TkTopLevel {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl TkTopLevel {

    /// Specifies the background colour.
    pub fn background(&self, colour: &str) {
        widget::configure(&self.id, "background", colour);
    }

    /// Size of border around widget.
    pub fn border_width(&self, width: u32) {
        widget::configure(&self.id, "borderwidth", &width.to_string());
    }

    /// De-iconify the window.
    pub fn deiconify(&self) {
        let msg = format!("wm deiconify {}", self.id);
        wish::tell_wish(&msg);
    }

    /// Expand window to occupy the full screen.
    pub fn full_screen(&self) {
        let msg = format!("wm attributes {} -fullscreen 1", self.id);
        wish::tell_wish(&msg);
    }

    /// Retrieves the geometry of the window.
    ///
    /// TODO: parse out of tcl format
    pub fn geometry(&self) -> String {
        let msg = format!("puts [wm geometry {}] ; flush stdout", self.id);
        wish::eval_wish(&msg)
    }

    /// Sets the size and position of a top-level window.
    ///
    /// * `height` - the vertical height of the window, in pixels
    /// * `width` - the horizontal width of the window, in pixels
    /// * `x` - a positive value gives position relative to _left_ edge of screen,
    ///         and a negative value gives position relative to _right_ edge.
    /// * `y` - a positive value gives position relative to _top_ edge of screen,
    ///         and a negative value gives position relative to _bottom_ edge.
    pub fn geometry_set(&self, width: u32, height: u32, x: i32, y: i32) {
        let msg = format!("wm geometry {} {}x{}{}{}{}{}",
                          self.id,
                          width, height,
                          if x < 0 { "-" } else { "+" },
                          x.abs(),
                          if y < 0 { "-" } else { "+" },
                          y.abs());
        wish::tell_wish(&msg);
    }

    /// Height of window, in rows.
    pub fn height(&self, height: u32) {
        widget::configure(&self.id, "height", &height.to_string());
    }

    /// Iconify the window.
    pub fn iconify(&self) {
        let msg = format!("wm iconify {}", self.id);
        wish::tell_wish(&msg);
    }

    /// Sets the maximum width/height in pixels for the window.
    pub fn maximum_size(&self, width: u32, height: u32) {
        let msg = format!("wm maxsize {} {} {}", self.id, width, height);
        wish::tell_wish(&msg);
    }

    /// Sets the menu of the top-level window.
    pub fn menu(&self, menu: &menu::TkMenu) {
        widget::configure(&self.id, "menu", &menu.id);
    }

    /// Sets the minimum width/height in pixels for the window.
    pub fn minimum_size(&self, width: u32, height: u32) {
        let msg = format!("wm minsize {} {} {}", self.id, width, height);
        wish::tell_wish(&msg);
    }

    /// Call given command on closing the window.
    pub fn on_close(&self, command: impl Fn()->() + Send + 'static) {
        wish::add_callback0(&self.id, wish::mk_callback0(command));
        let msg = format!("wm protocol {} WM_DELETE_WINDOW {{ puts clicked-{} ; flush stdout }}", 
                          self.id, self.id);
        wish::tell_wish(&msg);
    }

    /// Amount of horizontal padding for widget.
    pub fn padx(&self, value: u32) {
        widget::configure(&self.id, "padx", &value.to_string());
    }

    /// Amount of vertical padding for widget.
    pub fn pady(&self, value: u32) {
        widget::configure(&self.id, "pady", &value.to_string());
    }

    /// Style of border around label.
    pub fn relief(&self, value: widget::Relief) {
        widget::relief(&self.id, value);
    }

    /// Sets if window can be resized vertically or horizontally.
    pub fn resizable(&self, width: bool, height: bool) {
        let msg = format!("wm resizable {} {} {}",
                          self.id,
                          if width { "1" } else { "0" },
                          if height { "1" } else { "0" });
        wish::tell_wish(&msg);
    }

    /// Sets the title text on a top-level window.
    pub fn title(&self, title: &str) {
        let msg = format!("wm title {} {{{}}}\n", self.id, title);
        wish::tell_wish(&msg);
    }

    /// Updates the display.
    pub fn update_idle_tasks(&self) {
        wish::tell_wish("update idletasks");
    }

    /// Width of window, in columns.
    pub fn width(&self, width: u32) {
        widget::configure(&self.id, "width", &width.to_string());
    }

    /// Withdraw the window.
    pub fn withdraw(&self) {
        let msg = format!("wm withdraw {}", self.id);
        wish::tell_wish(&msg);
    }

 
}
