use afrish::*;
use std::sync::Arc;
use std::sync::Mutex;

fn result(answer: i32, 
          interrupt: Arc<Mutex<bool>>,
          progressbar: afrish::TkProgressbar, 
          button: afrish::TkButton, 
          label: afrish::TkLabel) {
    progressbar.value(0.0);
    if answer >= 0 {
        label.text(&format!("Answer {}", answer));
    } else {
        label.text("No answer");
    }
    button.text("Start!");
    button.clone().command(move || {
        start(interrupt.clone(), progressbar.clone(), button.clone(), label.clone());
    });
}

fn start(interrupt: Arc<Mutex<bool>>, 
         progressbar: afrish::TkProgressbar,
         button: afrish::TkButton,
         label: afrish::TkLabel) {
    button.text("Stop!");
    {
        let interrupt = interrupt.clone();
        button.clone().command(move || { stop(interrupt.clone()); });
    }
    label.text("Working ...");
    let mut interruptv = interrupt.lock().unwrap();
    *interruptv = false;

    {
        let interrupt = interrupt.clone();
        afrish::after(1, move || {
            step(0.0, interrupt.clone(), progressbar.clone(), button.clone(), label.clone());
        });
    }
}

fn step(count: f32, 
        interrupt: Arc<Mutex<bool>>, 
         progressbar: afrish::TkProgressbar,
         button: afrish::TkButton,
         label: afrish::TkLabel) {
    progressbar.value(count);
    let interruptv = interrupt.lock().unwrap();
    if *interruptv {
        result(-1, interrupt.clone(), progressbar.clone(), button.clone(), label.clone());
    } else {
        {
            let interrupt = interrupt.clone();
            afrish::after(100, move || {
                if count >= 20.0 {
                    result(42, interrupt.clone(), progressbar.clone(), button.clone(), label.clone());
                } else {
                    step(count+1.0, interrupt.clone(), progressbar.clone(), button.clone(), label.clone());
                }
            });
        }
    }
}

fn stop(interrupt: Arc<Mutex<bool>>) {
    let mut interrupt = interrupt.lock().unwrap();
    *interrupt = true;
}

fn main() {
    let root = afrish::start_wish().unwrap();
    root.title("event-example.rs");

    // - set up the interface
    let button = afrish::make_button(&root);
    button.text("Start!");
    let label = afrish::make_label(&root);
    label.text("No Answer");
    let progressbar = afrish::make_progressbar(&root, 
                                             afrish::Orientation::Horizontal,
                                             afrish::ProgressMode::Determinate);
    progressbar.maximum(20.0);

    button.grid().row(0).column(1).padx(5).pady(5).layout();
    label.grid().row(0).column(0).padx(5).pady(5).layout();
    progressbar.grid().row(1).column(0).padx(5).pady(5).layout();

    // - functions
    let interrupt = Arc::new(Mutex::new(false));
    button.clone().command(move || {
        start(interrupt.clone(), progressbar.clone(), button.clone(), label.clone());
    });

    afrish::mainloop();
}
