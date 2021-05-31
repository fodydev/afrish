//! Treeview widget - TO COMPLETE
//!
//! A combined tree/list-view widget, for displaying hierarchical data with 
//! multiple values.
//!
//! * also see the Tk [manual](http://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_treeview.htm)

use super::grid;
use super::widget;
use super::wish;

/// Refers to a treeview widget 
#[derive(Clone,Debug,PartialEq)]
pub struct TkTreeview {
    pub id: String,
}

/// Refers to a treeview item 
#[derive(Clone,Debug,PartialEq)]
pub struct TkTreeviewItem {
    pub treeview: String,
    pub id: String,
}

/// Creates an instance of a treeview widget in given parent.
pub fn make_treeview(parent: &impl widget::TkWidget) -> TkTreeview {
    let id = wish::next_wid(parent.id());
    let msg = format!("ttk::treeview {}", id);
    wish::tell_wish(&msg);

    TkTreeview {
        id,
    }
}

impl widget::TkWidget for TkTreeview {
    /// Returns the widget's id reference - used within tk
    fn id(&self) -> &str {
        &self.id
    }
}

impl grid::TkGridLayout for TkTreeview {
}

impl TkTreeview {
    /// Create a new top-level item at end of treeview.
    pub fn insert_item(&self) -> TkTreeviewItem {
        let msg = format!("{} insert {{}} end", &self.id);
        let result = wish::eval_wish(&msg);

        TkTreeviewItem {
            treeview: self.id.clone(),
            id: result,
        }
    }

    /// Create a new top-level item at given index position of treeview.
    pub fn insert_item_at(&self, index: u32) -> TkTreeviewItem {
        let msg = format!("{} insert {{}} {}", &self.id, index);
        let result = wish::eval_wish(&msg);

        TkTreeviewItem {
            treeview: self.id.clone(),
            id: result,
        }
    }

    /// Moves a given item to become a child of given parent.
    pub fn move_item(&self, 
                     child: &TkTreeviewItem, 
                     parent: &TkTreeviewItem, 
                     index: u32) {
        let msg = format!("{} move {} {} {}",
                          &self.id, &child.id, &parent.id, index);
        wish::tell_wish(&msg);
    }
}

impl TkTreeviewItem {
    /// Deletes this widget from tree.
    pub fn delete(&self) {
        let msg = format!("{} delete {}",
                          &self.treeview, &self.id);
        wish::tell_wish(&msg);
    }

    /// Sets the text label for the item.
    pub fn text(&self, value: &str) {
        let msg = format!("{} item {} -text {{{}}}",
                          &self.treeview, &self.id, value);
        wish::tell_wish(&msg);
    }

    /// Sets an image to display on the item.
    pub fn image(&self, image: &image::TkImage) {
        let msg = format!("{} item {} -image {}",
                          &self.treeview, &self.id, &image.id);
        wish::tell_wish(&msg);
    }

    /// The list of values to display for this item.
    pub fn values(&self, values: &[&str]) {
        let mut values_str = String::from("");
        for value in values {
            values_str.push('{');
            values_str.push_str(value);
            values_str.push('}');
            values_str.push(' ');
        }

        let msg = format!("{} item {} -values {{{}}}",
                          &self.treeview, &self.id, values_str);
        wish::tell_wish(&msg);
    }

    /// Sets item state to be open or closed.
    pub fn open(&self, value: true) {
        let msg = format!("{} item {} -open {}",
                          &self.treeview, &self.id,
                          if value { "1" } else { "0" });
        wish::tell_wish(&msg);
    }

    /// Returns true/false if item is open or closed.
    pub fn is_open(&self) -> bool {
        let msg = format!("{} item {} -open",
                          &self.treeview, &self.id);
        let result = wish::eval_wish(&msg);

        result == "1"
    }

    /// Create a new item at end of this treeview item.
    pub fn insert_item(&self) -> TkTreeviewItem {
        let msg = format!("{} insert {} end ", &self.treeview, &self.id);
        let result = wish::eval_wish(&msg);

        TkTreeviewItem {
            treeview: self.id.clone(),
            id: result,
        }
    }

    /// Create a new top-level item at given index position of this 
    /// treeview item.
    pub fn insert_item_at(&self, index: u32) -> TkTreeviewItem {
        let msg = format!("{} insert {} {}", &self.treeview, &self.id, index);
        let result = wish::eval_wish(&msg);

        TkTreeviewItem {
            treeview: self.id.clone(),
            id: result,
        }
    }

    /// Returns an Option type containing the parent item if found, or 
    /// None if this is a top-level item.
    pub fn parent(&self) -> Option<TkTreeviewItem> {
        let msg = format!("puts [{} parent {}] ; flush stdout",
                          &self.treeview, &self.id);
        let result = wish::eval_wish(&msg);

        if result == "" || result == "{}" {
            None
        } else {
            Some(TkTreeviewItem {
                treeview: self.treeview.clone(),
                id: result,
            })
        }
    }

    /// Returns an Option type containing the previous item to 
    /// this one in its list, if found, or None if this is the 
    /// first child of its parent.
    pub fn previous(&self) -> Option<TkTreeviewItem> {
        let msg = format!("puts [{} prev {}] ; flush stdout",
                          &self.treeview, &self.id);
        let result = wish::eval_wish(&msg);

        if result == "" || result == "{}" {
            None
        } else {
            Some(TkTreeviewItem {
                treeview: self.treeview.clone(),
                id: result,
            })
        }
    }

    /// Returns an Option type containing the next item to 
    /// this one in its list, if found, or None if this is the 
    /// last child of its parent.
    pub fn next(&self) -> Option<TkTreeviewItem> {
        let msg = format!("puts [{} next {}] ; flush stdout",
                          &self.treeview, &self.id);
        let result = wish::eval_wish(&msg);

        if result == "" || result == "{}" {
            None
        } else {
            Some(TkTreeviewItem {
                treeview: self.treeview.clone(),
                id: result,
            })
        }
    }

    /// Returns a list of child items of given node.
    pub fn children(&self) -> Vec<TkTreeviewItem> {
        let msg = format!("puts [{} children {}] ; flush stdout",
                          &self.treeview, &self.id);
        let result = wish::eval_wish(&msg);

        let mut children: Vec<TkTreeviewItem> = vec![];

        for child in result.split_whitespace() {
            children.push(TkTreeviewItem {
                treeview: self.treeview.clone(),
                id: String::from(child),
            });
        }

        children
    }
}
