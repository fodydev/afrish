//! Menu widget
//!
//! The menu widget is a versatile widget. It can take on any of these 
//! roles:
//!
//! * a window's menubar, containing several menus, such as "File", "Edit", 
//!   "Help" etc. 
//! * a menu containing menu items, such as "Open File", or submenus, such 
//!   as "Recent files...".
//! * a menu button, which can be clicked to invoke an action. This button 
//!   can act as a button, a check button, or a radio button.
//!
//! This library uses separate structs for the different types of menu which can 
//! be added to an existing menu. These are built up in standard "builder" 
//! style, before being added or inserted.
//! 
//! * also see the Tk [manual](http://www.tcl-lang.org/man/tcl8.6/TkCmd/menu.htm)
//!

use super::image;
use super::toplevel;
use super::widget;
use super::wish;

/// Refers to a menu widget
#[derive(Clone)]
pub struct TkMenu {
    pub id: String,
}

/// Creates an instance of a menu widget in given parent. 
pub fn make_menu(parent: &toplevel::TkTopLevel) -> TkMenu {
    let id = wish::next_wid(&parent.id);
    let msg = format!("menu {}", id);
    wish::tell_wish(&msg);

    TkMenu {
        id,
    }
}

// -- subtypes of menu

/// Refers to a submenu
#[derive(Clone)]
pub struct TkMenuCascade {
    parent: String,
    compound: widget::Compound,
    font: Option<String>,
    image: Option<String>,
    label: Option<String>,
    menu: Option<String>,
    underline: Option<u32>,
}

/// Refers to a check-button
#[derive(Clone)]
pub struct TkMenuCheck {
    parent: String,
}

/// Refers to a command (menu-item)
#[derive(Clone)]
pub struct TkMenuCommand {
    parent: String,
    accelerator: Option<String>,
    command: Option<String>,
    compound: widget::Compound,
    font: Option<String>,
    image: Option<String>,
    label: Option<String>,
    underline: Option<u32>,
}

/// Refers to a radio-button
#[derive(Clone)]
pub struct TkMenuRadio {
    parent: String,
}

/// Refers to a menu separator
#[derive(Clone)]
pub struct TkMenuSeparator {
    parent: String,
}

// -- implementations of each menu type

impl widget::TkWidget for TkMenu {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl TkMenu {
    /// The accelerator is a key-stroke linked (by "bind") to the 
    /// action for this menuitem.
    pub fn accelerator(&self, key_definition: &str) {
        widget::configure(&self.id, "accelerator", key_definition);
    }

    /// Sets image for menu.
    pub fn image(&self, image: &image::TkImage) {
        widget::configure(&self.id, "image", &image.id);
    }
    
    /// Sets text label for menu.
    pub fn label(&self, text: &str) {
        widget::configure(&self.id, "label", text);
    }

    /// Start to create a submenu.
    pub fn cascade(&self) -> TkMenuCascade {
        TkMenuCascade {
            parent: self.id.clone(),
            compound: widget::Compound::None,
            font: None,
            image: None,
            label: None,
            menu: None,
            underline: None,
        }
    }

    /// Start to create a check-button menu-item.
    pub fn check_button(&self) -> TkMenuCheck {
        TkMenuCheck {
            parent: self.id.clone(),
        }
    }

    /// Start to create a command (simple menu item).
    pub fn command(&self) -> TkMenuCommand {
        TkMenuCommand {
            parent: self.id.clone(),
            accelerator: None,
            command: None,
            compound: widget::Compound::None,
            font: None,
            image: None,
            label: None,
            underline: None,
        }
    }

    /// Start to create a radio-button menu-item.
    pub fn radio_button(&self) -> TkMenuRadio {
        TkMenuRadio {
            parent: self.id.clone(),
        }
    }

    /// Start to create a separator.
    pub fn separator(&self) -> TkMenuSeparator {
        TkMenuSeparator {
            parent: self.id.clone(),
        }
    }
}

impl TkMenuCascade {
        /// Sets arrangement if menu item has both an image and a label.
    pub fn compound(&mut self, value: widget::Compound) -> &mut Self {
        self.compound = value;
        self
    }

    /// Sets font.
    pub fn font(&mut self, value: &str) -> &mut Self {
        self.font = Some(String::from(value));
        self
    }

    /// Sets image to display.
    pub fn image(&mut self, image: &image::TkImage) -> &mut Self {
        self.image = Some(String::from(&image.id));
        self
    }

    /// Sets text label to display.
    pub fn label(&mut self, value: &str) -> &mut Self {
        self.label = Some(String::from(value));
        self
    }

    /// Sets submenu to display.
    pub fn menu(&mut self, submenu: &TkMenu) -> &mut Self {
        self.menu = Some(submenu.id.clone());
        self
    }

    /// Sets index of text label to underline.
    pub fn underline(&mut self, value: u32) -> &mut Self {
        self.underline = Some(value);
        self
    }

    // Convert options into a string representation
    fn option_string(&self) -> String {
        let mut msg = String::from("");
        let compound = match self.compound {
            widget::Compound::Bottom => "bottom",
            widget::Compound::Center | widget::Compound::Centre => "center",
            widget::Compound::Image => "image",
            widget::Compound::Left => "left",
            widget::Compound::None => "none",
            widget::Compound::Right => "right",
            widget::Compound::Text => "text",
            widget::Compound::Top => "top",
        };
        msg.push_str(&format!("-compound {} ", compound));
        if let Some(font) = &self.font {
            msg.push_str(&format!("-font {{{}}} ", font));
        }
        if let Some(image) = &self.image {
            msg.push_str(&format!("-image {{{}}} ", image));
        }
        if let Some(label) = &self.label {
            msg.push_str(&format!("-label {{{}}} ", label));
        }
        if let Some(menu) = &self.menu {
            msg.push_str(&format!("-menu {} ", menu));
        }
        if let Some(underline) = &self.underline {
            msg.push_str(&format!("-underline {} ", underline));
        }

        msg
    }

    /// Adds cascade menu-item to parent with current set of options.
    pub fn add(&self) {
        let msg = format!("{} add cascade {}", &self.parent, &self.option_string());
        wish::tell_wish(&msg);
    }
    
    /// Inserts cascade menu-item to parent at given index with current set of options.
    pub fn insert(&self, index: u32) {
        let msg = format!("{} insert cascade {} {}", 
                          &self.parent, index, &self.option_string());
        wish::tell_wish(&msg);
    }
}

impl TkMenuCheck {
}

impl TkMenuCommand {
    /// Sets value of accelerator key - displayed to right of menu.
    pub fn accelerator(&mut self, key_definition: &str) -> &mut Self {
        self.accelerator = Some(String::from(key_definition));
        self
    }

    /// Sets command to invoke when menu-item clicked.
    pub fn command(&mut self, command: impl Fn()->() + Send + 'static) -> &mut Self {
        let id = wish::next_wid(".");
        wish::add_callback0(&id, wish::mk_callback0(command));
        self.command = Some(String::from(id));
        self
    }

    /// Sets arrangement if menu item has both an image and a label.
    pub fn compound(&mut self, value: widget::Compound) -> &mut Self {
        self.compound = value;
        self
    }

    /// Sets font.
    pub fn font(&mut self, value: &str) -> &mut Self {
        self.font = Some(String::from(value));
        self
    }

    /// Sets image to display.
    pub fn image(&mut self, image: &image::TkImage) -> &mut Self {
        self.image = Some(String::from(&image.id));
        self
    }

    /// Sets text label to display.
    pub fn label(&mut self, value: &str) -> &mut Self {
        self.label = Some(String::from(value));
        self
    }

    /// Sets index of text label to underline.
    pub fn underline(&mut self, value: u32) -> &mut Self {
        self.underline = Some(value);
        self
    }

    // Convert options into a string representation
    fn option_string(&self) -> String {
        let mut msg = String::from("");
        if let Some(accelerator) = &self.accelerator {
            msg.push_str(&format!("-accelerator {{{}}} ", &accelerator));
        }
        if let Some(command) = &self.command {
            msg.push_str(&format!("-command {{ puts clicked-{} ; flush stdout }} ", 
                                  &command));
        }
        let compound = match self.compound {
            widget::Compound::Bottom => "bottom",
            widget::Compound::Center | widget::Compound::Centre => "center",
            widget::Compound::Image => "image",
            widget::Compound::Left => "left",
            widget::Compound::None => "none",
            widget::Compound::Right => "right",
            widget::Compound::Text => "text",
            widget::Compound::Top => "top",
        };
        msg.push_str(&format!("-compound {} ", compound));
        if let Some(font) = &self.font {
            msg.push_str(&format!("-font {{{}}} ", font));
        }
        if let Some(image) = &self.image {
            msg.push_str(&format!("-image {{{}}} ", image));
        }
        if let Some(label) = &self.label {
            msg.push_str(&format!("-label {{{}}} ", label));
        }
        if let Some(underline) = &self.underline {
            msg.push_str(&format!("-underline {} ", underline));
        }

        msg
    }

    /// Adds command menu-item to parent with current set of options.
    pub fn add(&self) {
        let msg = format!("{} add command {}", &self.parent, &self.option_string());
        wish::tell_wish(&msg);
    }
    
    /// Inserts command menu-item to parent at given index with current set of options.
    pub fn insert(&self, index: u32) {
        let msg = format!("{} insert command {} {}", 
                          &self.parent, index, &self.option_string());
        wish::tell_wish(&msg);
    }
}

impl TkMenuRadio {
}

impl TkMenuSeparator {
    /// Adds separator to the parent menu.
    pub fn add(&self) {
        let msg = format!("{} add separator", &self.parent);
        wish::tell_wish(&msg);
    }
}
