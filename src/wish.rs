//! Core functions and data structures for interacting with the wish process.
//!

use std::collections::HashMap;
use std::sync::mpsc;
use std::io::{Read, Write};
use std::process;
use std::thread;
use super::toplevel;

use once_cell::sync::OnceCell;

static mut WISH: OnceCell<process::Child> = OnceCell::new();
static mut OUTPUT: OnceCell<process::ChildStdout> = OnceCell::new();
static mut SENDER: OnceCell<mpsc::Sender<String>> = OnceCell::new();

pub(super) fn kill_wish() {
    unsafe {
        WISH.get_mut().unwrap().kill().expect("Wish was unexpectedly already finished");
    }
}

pub(super) fn tell_wish(msg: &str) {
    unsafe {
        SENDER.get_mut().unwrap().send(String::from(msg)).unwrap();
        SENDER.get_mut().unwrap().send(String::from("\n")).unwrap();
    }
}

// -- Counter for making new ids

static mut NEXT_ID: i32 = 0;

pub(super) fn next_wid(parent: &str) -> String {
    unsafe {
        NEXT_ID += 1;
        if parent == "." {
            format!(".r{}", NEXT_ID)
        } else {
            format!("{}.r{}", parent, NEXT_ID)
        }
    }
}

pub(super) fn current_id() -> i32 {
    unsafe {
        NEXT_ID
    }
}

// -- Store for callback functions, such as on button clicks

type Callback0 = Box<(dyn Fn()->() + 'static)>;
pub(super) fn mk_callback0<F>(f: F) -> Callback0
    where F: Fn()->() + 'static {
        Box::new(f) as Callback0
}

static mut CALLBACKS0: OnceCell<HashMap<String, Callback0>> = OnceCell::new();

pub(super) fn add_callback0(wid: &str, callback: Callback0) {
    unsafe {
        CALLBACKS0.get_mut().unwrap().insert(String::from(wid), callback);
    }
}

fn eval_callback0(wid: &str) {
    unsafe {
        if let Some(command) = CALLBACKS0.get_mut().unwrap().get(wid) {
            command();
        } // TODO - error?
    }
}

type Callback1Bool = Box<(dyn Fn(bool)->() + 'static)>;
pub(super) fn mk_callback1_bool<F>(f: F) -> Callback1Bool
    where F: Fn(bool)->() + 'static {
        Box::new(f) as Callback1Bool
}

static mut CALLBACKS1BOOL: OnceCell<HashMap<String, Callback1Bool>> = OnceCell::new();

pub(super) fn add_callback1_bool(wid: &str, callback: Callback1Bool) {
    unsafe {
        CALLBACKS1BOOL.get_mut().unwrap().insert(String::from(wid), callback);
    }
}

fn eval_callback1_bool(wid: &str, value: bool) {
    unsafe {
        if let Some(command) = CALLBACKS1BOOL.get_mut().unwrap().get(wid) {
            command(value);
        } // TODO - error?
    }
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
                    } else if input.starts_with("cb1") { // -- callback 1
                        let parts: Vec<&str> = input.split('-').collect();
                        let widget = parts[1].trim();
                        let value = parts[2].trim();
                        println!("Callback on |{}| with |{}|", widget, value);
                        eval_callback1_bool(widget, value=="1");
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

/// Creates a connection with the "wish" program
pub fn start_wish () -> toplevel::TkTopLevel {
    unsafe {
        WISH.set(process::Command::new("wish")
                 .stdin(process::Stdio::piped())
                 .stdout(process::Stdio::piped())
                 .spawn()
                 .expect("failed to execute"))
            .expect("Do not start wish twice");

        let mut input = WISH.get_mut().unwrap().stdin.take().unwrap(); 
        OUTPUT.set(WISH.get_mut().unwrap().stdout.take().unwrap())
            .expect("Do not start wish twice");

        input.write(b"package require Tcl\n").unwrap();
        input.write(b"wm protocol . WM_DELETE_WINDOW { puts stdout {exit} ; flush stdout } \n").unwrap();

        let (sender, receiver) = mpsc::channel();
        SENDER.set(sender).expect("Do not start wish twice");

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
        CALLBACKS0.set(HashMap::new());
        CALLBACKS1BOOL.set(HashMap::new());
    }

    toplevel::TkTopLevel {
        id: String::from("."),
    }
}

