# CLI Clipboard

![Rust](https://github.com/TheKiteEatingTree/cli-clipboard/workflows/Rust/badge.svg)

cli-clipboard is a fork of [rust-clipboard](https://github.com/aweinstock314/rust-clipboard) that adds wayland support for terminal and window-less applications via [wl-clipboard-rs](https://github.com/YaLTeR/wl-clipboard-rs). For terminal applications it supports copy and paste for both wayland and X11 linux environments, macOS and windows.

On Linux it will first attempt to setup a Wayland clipboard provider.  If that fails it will then fallback to the X11 clipboard provider.

## Examples

Using ClipboardContext to create a clipboard provider:

```rust
use cli_clipboard::{ClipboardContext, ClipboardProvider};

let mut ctx = ClipboardContext::new().unwrap();
let the_string = "Hello, world!";
ctx.set_contents(the_string.to_owned()).unwrap();
assert_eq!(ctx.get_contents().unwrap(), the_string);
ctx.clear();
// clearing the clipboard causes get_contents to return Err on macos and windows
if cfg!(any(windows, target_os = "macos")) {
    if ctx.get_contents().is_ok() {
        panic!("Should be Err");
    }
} else {
    assert_eq!(ctx.get_contents(), "");
}
```

Using the helper functions:

```rust
use cli_clipboard;

let the_string = "Hello, world!";
cli_clipboard::set_contents(the_string.to_owned()).unwrap();
assert_eq!(cli_clipboard::get_contents().unwrap(), the_string);
```

## API

### ClipboardProvider

The `ClipboardProvider` trait has the following functions:

```rust
fn new() -> anyhow::Result<Self>;
fn get_contents(&mut self) -> anyhow::Result<String>;
fn set_contents(&mut self, String) -> anyhow::Result<()>;
fn clear(&mut self) -> anhow::Result<()>;
```

### ClipboardContext

- `ClipboardContext` is a type alias for one of {`WindowsClipboardContext`, `OSXClipboardContext`, `LinuxClipboardContext`}, all of which implement `ClipboardProvider`. Which concrete type is chosen for `ClipboardContext` depends on the OS (via conditional compilation). 
- `WaylandClipboardContext` and `X11ClipboardContext` are also available but generally the correct one will be chosen by `LinuxClipboardContext`.

### Convenience Functions

`get_contents` and `set_contents` are convenience functions that create a context for you and call the respective function on it.

## Alternatives

1. [copypasta - rust-clipboard fork adding wayland support for windowed applications](https://github.com/alacritty/copypasta)
1. [The original rust-clipboard](https://github.com/aweinstock314/rust-clipboard)

## License

`cli-clipboard` is dual-licensed under MIT and Apache2.
