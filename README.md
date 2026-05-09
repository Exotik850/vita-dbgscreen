# vita-dbgscreen

A simple Rust crate intended for rendering a debug screen on the PlayStation Vita. Built on top of `vitasdk-sys`, it provides an easy way to output debug text and information directly to the Vita's display during homebrew development.

## Usage

First, add `vita-dbgscreen` to your `Cargo.toml`.

Then, you can use it in your code as follows:

```rust
use core::fmt::Write;
use vita_dbgscreen::DebugScreen;

// Initialize the debug screen
let mut dbgscreen = DebugScreen::new();

// Since DebugScreen implements core::fmt::Write, 
// you can use standard macros like write! and writeln!
writeln!(dbgscreen, "Hello World from vita-dbgscreen!").unwrap();
```

There's also a basic panic handler included that will display panic messages on the debug screen. To use it, simply set it as the panic handler in your `main.rs`:

```rust
use vita_dbgscreen::set_dbg_screen_panic_handler;
```