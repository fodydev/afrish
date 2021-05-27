//! A Rust binding for the Tk graphics toolkit.
//!
//! rstk opens and communicates with Tk's wish program as a separate process.
//! The library provides:
//! 
//! * low-level functions to directly communicate with wish, suitable for 
//!   writing additional extensions
//! * high-level API to write GUI applications with minimal knowledge of Tk.
//!
//! The top-level functions to start/stop the GUI are contained 
//! in the [wish] module.
//!
//! The remaining modules describe a widget or supporting component
//! (such as a font or image). Within these modules, functions usually
//! act as constructors/modifiers for the widget or component. 
//!
//! Click on the struct name to get a list of methods supported by the 
//! widget; functionality is divided between various traits.
//!
//! # Example
//!
//! A simple hello-world example:
//! 
//! ```
//! use rstk;
//! 
//! fn main() {
//!   let root = rstk::start_wish();
//! 
//!   let hello = rstk::make_label(&root);
//!   hello.text("Hello from Rust/Tk");
//! 
//!   hello.grid().layout();
//! 
//!   rstk::mainloop();
//! }
//! ```
//!
//! For more examples and documentation, see the project 
//! [webpage](https://peterlane.netlify.app/rstk/).
//! 
//! ## Low-level API 
//! 
//! The main wrapper aims to provide a rust-friendly, type-checked set of structs 
//! and methods for using the Tk library.
//! 
//! However, there are many features in Tk and not all of it is likely to be
//! wrapped. If there is a feature missing, it is possible to directly use Tk
//! commands to access it.
//! 
//! 1. every widget has an `id` field, which gives the Tk identifier. 
//! 2. [tell_wish](wish::tell_wish) sends a given string directly to wish
//! 3. [eval_wish](wish::eval_wish) sends a given string directly to wish and 
//!    returns, as a [String], the response.
//! 
//! For example, label's
//! [takefocus](http://www.tcl-lang.org/man/tcl8.6/TkCmd/ttk_widget.htm#M-takefocus)
//! flag is not wrapped. You can nevertheless set it using:
//! 
//! ```
//! let label = rstk::make_label(&root);
//! 
//! rstk::tell_wish(&format!("{} configure -takefocus 0", label.id));
//! ```
//! 
//! Also useful are:
//! 
//! * [cget](widget::TkWidget::cget) - queries any option and returns its current value
//! * [configure](widget::TkWidget::configure) - used to set any option to a value
//! * [winfo](widget::TkWidget::winfo) - returns window-related information
//! 
//! ## Extensions
//! 
//! Extensions can be created with the help of [next_wid](wish::next_wid), 
//! which returns a new, unique ID in Tk format. Writing an extension requires:
//! 
//! 1. importing the tcl/tk library (using `tell_wish`)
//! 2. creating an instance of the underlying Tk widget using a unique id
//! 3. retaining that id in a struct, for later reference
//! 4. wrapping the widget's functions as methods, calling out to Tk with
//!    the stored id as a reference.

pub mod button;
pub use button::*;

pub mod canvas;
pub use canvas::*;

pub mod check_button;
pub use check_button::*;

pub mod combobox;
pub use combobox::*;

pub mod dialog;
pub use dialog::*;

pub mod entry;
pub use entry::*;

pub mod font;
pub use font::*;

pub mod frame;
pub use frame::*;

pub mod grid;
pub use grid::*;

pub mod image;
pub use image::*;

pub mod label;
pub use label::*;

pub mod label_frame;
pub use label_frame::*;

pub mod menu;
pub use menu::*;

pub mod notebook;
pub use notebook::*;

pub mod paned_window;
pub use paned_window::*;

pub mod radio_button;
pub use radio_button::*;

pub mod separator;
pub use separator::*;

pub mod theme;
pub use theme::*;

pub mod toplevel;
pub use toplevel::*;

pub mod widget;
pub use widget::*;

pub mod wish;
pub use wish::*;

