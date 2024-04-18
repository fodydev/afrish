# rish

A Rust binding for the Tk graphics toolkit.

## Overview

rish opens and communicates with Tk's wish program as a separate process.
The library provides:

* low-level functions to directly communicate with wish, suitable for 
  writing additional extensions
* high-level API to write GUI applications with minimal knowledge of Tk.

## Example

A simple hello-world example:

```rust
use rish::*;

fn main() {
  let root = rish::start_wish().unwrap();

  let hello = rish::make_label(&root);
  hello.text("Hello from Rust/Tk");

  hello.grid().layout();

  rish::mainloop();
}
```

## Credits
This project is a clone of [rstk](https://codeberg.org/peterlane/rstk).