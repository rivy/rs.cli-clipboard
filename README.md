# CLI Clipboard

![Rust](https://github.com/TheKiteEatingTree/cli-clipboard/workflows/Rust/badge.svg)

cli-clipboard is a fork of [rust-clipboard](https://github.com/aweinstock314/rust-clipboard) that adds wayland support for terminal and window-less applications via [wl-clipboard-rs](https://github.com/YaLTeR/wl-clipboard-rs). For terminal applications it supports copy and paste for both wayland and X11 linux environments, macOS and windows.

Consider this alpha software.  The tests pass on linux, macOS and windows but it has not yet been manually tested on every platform.

## Example

```rust
use cli_clipboard;

fn example() {
    cli_clipboard::set_contents("some string".to_owned()).unwrap();
    assert_eq!(cli_clipboard::get_contents().unwrap(), "some string");
}
```

## API

### ClipboardProvider

The `ClipboardProvider` trait has the following functions:

```rust
fn new() -> Result<Self, Box<Error>>;
fn get_contents(&mut self) -> Result<String, Box<Error>>;
fn set_contents(&mut self, String) -> Result<(), Box<Error>>;
```

### ClipboardContext

- `ClipboardContext` is a type alias for one of {`WindowsClipboardContext`, `OSXClipboardContext`, `X11ClipboardContext`, `NopClipboardContext`}, all of which implement `ClipboardProvider`. Which concrete type is chosen for `ClipboardContext` depends on the OS (via conditional compilation). 
- `WaylandClipboardContext` is also available but is never assigned to `ClipboardContext`.

### Convenience Functions

`get_contents` and `set_contents` are convenience functions that create a context for you and call the respective function on it. They correctly work on linux by attempting to create a wayland context and falling back to X11 if an error occurs.

## Alternatives

1. [copypasta - rust-clipboard fork adding wayland support for windowed applications](https://github.com/alacritty/copypasta)
1. [The original rust-clipboard](https://github.com/aweinstock314/rust-clipboard)

## License

`cli-clipboard` is dual-licensed under MIT and Apache2.
