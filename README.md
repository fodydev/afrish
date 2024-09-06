# afrish

A Rust binding for the Tk graphics toolkit, designed specifically for the development of the [Afrim IME](https://github.com/fodydev/afrim).

## Overview

afrish opens and communicates with Tk's wish program as a separate process.
The library provides:

* low-level functions to directly communicate with wish, suitable for 
  writing additional extensions
* high-level API to write GUI applications with minimal knowledge of Tk.

## Example

A simple hello-world example:

```rust
use afrish::*;

fn main() {
  let root = afrish::start_wish().unwrap();

  let hello = afrish::make_label(&root);
  hello.text("Hello from Rust/Tk");

  hello.grid().layout();

  afrish::mainloop();
}
```

## Credits
This project is a clone of [rstk](https://codeberg.org/peterlane/rstk).