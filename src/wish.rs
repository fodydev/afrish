//! Core functions and data structures for interacting with the wish process.
//!
//! The basic structure of a program using wish is as follows:
//!
//! ```
//! fn main() {
//!   let root = rstk::start_wish();
//!
//!   // -- add code here to create program
//!
//!   rstk::mainloop();
//! }
//! ```
//!
//! The call to `start_wish` starts the "wish" program and sets up some 
//! internal structure to store information about your program's interaction
//! with wish. 
//!
//! If you are using a different program to "wish", e.g. a tclkit, then 
//! call instead:
//!
//! ```
//!   let root = rst::start_with("tclkit");
//! ```
//! 
//! All construction of the GUI must be done after starting a wish process.
//!
//! Tk is event-driven, so the code sets up the content and design 
//! of various widgets and associates commands to particular events: events 
//! can be button-clicks or the movement of a mouse onto a canvas. 
//!
//! Once the GUI is created, then the `mainloop` must be started, which will 
//! process and react to events: the call to `mainloop` is usually the last 
//! statement in the program.
//! 
//! The other modules within "rstk" provide implementations of widgets, 
//! including buttons, labels, menus, and text widgets, as well as associated
//! data as fonts and images.
//!
//! ## Themes
//!
//! This module includes some functions to list and set the overall theme
//! (look and feel) of the Tk program.
//! 

use std::collections::HashMap;
use std::sync::mpsc;
use std::io::{Read, Write};
use std::process;
use std::sync::Mutex;
use std::thread;

use super::font;
use super::toplevel;
use super::widget;

// TODO - change when available from 'nightly'
use once_cell::sync::Lazy; 
use once_cell::sync::OnceCell;

static mut WISH: OnceCell<process::Child> = OnceCell::new();
static mut OUTPUT: OnceCell<process::ChildStdout> = OnceCell::new();
static mut SENDER: OnceCell<mpsc::Sender<String>> = OnceCell::new();

// Kills the wish process - should be called to exit
pub(super) fn kill_wish() {
    unsafe {
        WISH.get_mut().unwrap().kill().expect("Wish was unexpectedly already finished");
    }
}

/// Sends a message (tcl command) to wish.
///
/// Use with caution: the message must be valid tcl.
///
pub fn tell_wish(msg: &str) {
    unsafe {
        SENDER.get_mut().unwrap().send(String::from(msg)).unwrap();
        SENDER.get_mut().unwrap().send(String::from("\n")).unwrap();
    }
}

/// Sends a message (tcl command) to wish and expects a result.
/// Returns a result as a string
///
/// Use with caution: the message must be valid tcl.
///
pub fn eval_wish(msg: &str) -> String {
    tell_wish(msg);
    
    unsafe {
        let mut input = [32; 100];
        if let Ok(_) = OUTPUT.get_mut().unwrap().read(&mut input) {
            if let Ok(input) = String::from_utf8(input.to_vec()) {
                println!("Result {:?}", &input);
                return String::from(input).trim().to_string();
            }
        }
    }

    panic!("Eval-wish failed to get a result");
}

// -- Counter for making new ids

static NEXT_ID: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

/// Returns a new id string which can be used to name a new 
/// widget instance. The new id will be in reference to the 
/// parent, as is usual in Tk.
///
/// This is only for use when writing an extension library.
///
pub fn next_wid(parent: &str) -> String {
    let mut nid = NEXT_ID.lock().unwrap();
    *nid += 1;
    if parent == "." {
        format!(".r{}", nid)
    } else {
        format!("{}.r{}", parent, nid)
    }
}

pub(super) fn current_id() -> i32 {
    let nid = NEXT_ID.lock().unwrap();
    *nid
}

// -- Store for callback functions, such as on button clicks

type Callback0 = Box<(dyn Fn()->() + Send + 'static)>;
pub(super) fn mk_callback0<F>(f: F) -> Callback0
    where F: Fn()->() + Send + 'static {
        Box::new(f) as Callback0
}

static CALLBACKS0: Lazy<Mutex<HashMap<String, Callback0>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub(super) fn add_callback0(wid: &str, callback: Callback0) {
    CALLBACKS0.lock().unwrap().insert(String::from(wid), callback);
}

fn eval_callback0(wid: &str) {
    if let Some(command) = CALLBACKS0.lock().unwrap().get(wid) {
        command();
    } // TODO - error?
}

type Callback1Bool = Box<(dyn Fn(bool)->() + Send + 'static)>;
pub(super) fn mk_callback1_bool<F>(f: F) -> Callback1Bool
    where F: Fn(bool)->() + Send + 'static {
        Box::new(f) as Callback1Bool
}

static CALLBACKS1BOOL: Lazy<Mutex<HashMap<String, Callback1Bool>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub(super) fn add_callback1_bool(wid: &str, callback: Callback1Bool) {
    CALLBACKS1BOOL.lock().unwrap().insert(String::from(wid), callback);
}

fn eval_callback1_bool(wid: &str, value: bool) {
    if let Some(command) = CALLBACKS1BOOL.lock().unwrap().get(wid) {
        command(value);
    } // TODO - error?
}

type Callback1Event = Box<(dyn Fn(widget::TkEvent)->() + Send + 'static)>; 
pub(super) fn mk_callback1_event<F>(f: F) -> Callback1Event
where F: Fn(widget::TkEvent)->() + Send + 'static {
    Box::new(f) as Callback1Event
}

// for bound events, key is widgetid/all + pattern, as multiple events can be 
// bound to same entity
static CALLBACKS1EVENT: Lazy<Mutex<HashMap<String, Callback1Event>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub(super) fn add_callback1_event(wid: &str, callback: Callback1Event) {
    CALLBACKS1EVENT.lock().unwrap().insert(String::from(wid), callback);
}

fn eval_callback1_event(wid: &str, value: widget::TkEvent) {
    if let Some(command) = CALLBACKS1EVENT.lock().unwrap().get(wid) {
        command(value);
    } // TODO - error?
}

type Callback1Font = Box<(dyn Fn(font::TkFont)->() + Send + 'static)>; 
pub(super) fn mk_callback1_font<F>(f: F) -> Callback1Font
where F: Fn(font::TkFont)->() + Send + 'static {
    Box::new(f) as Callback1Font
}

static CALLBACKS1FONT: Lazy<Mutex<HashMap<String, Callback1Font>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub(super) fn add_callback1_font(wid: &str, callback: Callback1Font) {
    CALLBACKS1FONT.lock().unwrap().insert(String::from(wid), callback);
}

fn eval_callback1_font(wid: &str, value: font::TkFont) {
    if let Some(command) = CALLBACKS1FONT.lock().unwrap().get(wid) {
        command(value);
    } // TODO - error?
}

/// Loops while GUI events occur
pub fn mainloop () {
    unsafe {
        let mut counter = 1;
        loop {
            let mut input = [32; 100];
            if let Ok(_) = OUTPUT.get_mut().unwrap().read(&mut input) {
                if let Ok(input) = String::from_utf8(input.to_vec()) {
                    println!("Input {:?}", &input);

                    // here - do a match or similar on what was read from wish
                    if input.starts_with("clicked") { // -- callbacks
                        if let Some(n) = input.find('\n') {
                            let widget = &input[8..n];
                            println!("Callback on |{}|", widget);
                            eval_callback0(widget);
                        }

                    } else if input.starts_with("cb1b") { // -- callback 1 with bool
                        let parts: Vec<&str> = input.split('-').collect();
                        let widget = parts[1].trim();
                        let value = parts[2].trim();
                        println!("Callback on |{}| with |{}|", widget, value);
                        eval_callback1_bool(widget, value=="1");

                    } else if input.starts_with("cb1e") { // -- callback 1 with event
                        let parts: Vec<&str> = input.split(':').collect();
                        let widget_pattern = parts[1].trim();
                        println!("Callback on |{}| with event", widget_pattern);
                        let x = parts[2].parse::<i32>().unwrap_or(0);
                        let y = parts[3].parse::<i32>().unwrap_or(0);
                        let root_x = parts[4].parse::<i32>().unwrap_or(0);
                        let root_y = parts[5].parse::<i32>().unwrap_or(0);
                        let height = parts[6].parse::<i32>().unwrap_or(0);
                        let width = parts[7].parse::<i32>().unwrap_or(0);
                        let key_code = parts[8].parse::<u32>().unwrap_or(0);
                        let key_symbol = parts[9].parse::<String>().unwrap_or(String::from(""));
                        let mouse_button = parts[10].parse::<u32>().unwrap_or(0);
                        let event = widget::TkEvent {
                            x,
                            y,
                            root_x,
                            root_y,
                            height,
                            width,
                            key_code,
                            key_symbol,
                            mouse_button,
                        };
                        eval_callback1_event(widget_pattern, event);

                    } else if input.starts_with("font") { // -- callback 1 with font
                        let font = String::from(input[4..].trim());
                        println!("Callback with font |{}|", font);
                        eval_callback1_font("font", font::TkFont { description: font });

                    } else if input.starts_with("exit") { // -- wish has exited
                        println!("Counter: {}", counter);
                        kill_wish();
                        return; // exit loop and program
                    }
                }
                counter += 1;
            }
        }
    }
}

/// Creates a connection with the "wish" program.
pub fn start_wish () -> toplevel::TkTopLevel {
    start_with("wish")
}

/// Creates a connection with the given wish/tclkit program.
pub fn start_with(wish: &str) -> toplevel::TkTopLevel {
    let err_msg = format!("Do not start {} twice", wish);

    unsafe {
        WISH.set(process::Command::new(wish)
                 .stdin(process::Stdio::piped())
                 .stdout(process::Stdio::piped())
                 .spawn()
                 .expect("failed to execute"))
            .expect(&err_msg);

        let mut input = WISH.get_mut().unwrap().stdin.take().unwrap(); 
        OUTPUT.set(WISH.get_mut().unwrap().stdout.take().unwrap())
            .expect(&err_msg);

        input.write(b"package require Tcl\n").unwrap();
        input.write(b"wm protocol . WM_DELETE_WINDOW { puts stdout {exit} ; flush stdout } \n").unwrap();
        input.write(b"proc font_choice {w font args} {
            puts font$font
            flush stdout
        }\n").unwrap();

        let (sender, receiver) = mpsc::channel();
        SENDER.set(sender).expect(&err_msg);

        // create thread to receive strings to send on to wish
        thread::spawn(move || {
            loop { 
                let msg: Result<String, mpsc::RecvError> = receiver.recv();
                match msg {
                    Ok(msg) => {
                        input.write(msg.as_bytes()).unwrap();
                        input.write(b"\n").unwrap();
                    },
                    _ => { // ignore errors
                    },
                }
            }
        });
    }

    toplevel::TkTopLevel {
        id: String::from("."),
    }
}

/// Used to cleanly end the wish process and current rust program.
pub fn end_wish() {
    kill_wish();
    process::exit(0);
}

// -- independent functions

/// Returns a list of the current themes.
///
/// For example:
///
/// ```
/// let themes = rstk::theme_names();
/// println!("{} available themes: ", themes.len());
/// for theme in themes {
///     println!(" - {}", theme);
/// }
/// ```
/// 
/// Lists the themes (on Linux):
///
/// ```text
/// 4 available themes:
///  - clam
///  - alt
///  - default
///  - classic
/// ```
///
pub fn theme_names() -> Vec<String> {
    let themes = eval_wish("puts [ttk::style theme names] ; flush stdout");

    let mut result: Vec<String> = vec![];
    for theme in themes.split_whitespace() {
        result.push(String::from(theme));
    }

    result
}

/// Sets the current theme to the given theme-name
pub fn use_theme(name: &str) {
    let msg = format!("ttk::style theme use {}", name);
    tell_wish(&msg);
}
