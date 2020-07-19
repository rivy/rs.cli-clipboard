# Coach Clipboard

coach is a fork of [rust-clipboard](https://github.com/aweinstock314/rust-clipboard) that
pulls in some open pull requests adding wayland support and updating to rust 2018. This means that it
theoretically supports Windows, Mac OSX, Linux with Wayland or X11 and FreeBSD.

## Example

```rust
use clipboard;

fn example() {
    clipboard::set_contents("some string".to_owned()).unwrap();
    assert_eq!(clipboard::get_contents().unwrap(), "some string");
}
```

## API

The `ClipboardProvider` trait has the following functions:

```rust
fn new() -> Result<Self, Box<Error>>;
fn get_contents(&mut self) -> Result<String, Box<Error>>;
fn set_contents(&mut self, String) -> Result<(), Box<Error>>;
```

`ClipboardContext` is a type alias for one of {`WindowsClipboardContext`, `OSXClipboardContext`, `X11ClipboardContext`, `NopClipboardContext`}, all of which implement `ClipboardProvider`. Which concrete type is chosen for `ClipboardContext` depends on the OS (via conditional compilation). `WaylandClipboardContext` is also available but is never assigned to `ClipboardContext`.


`get_contents` and `set_contents` are convenience functions that create a context for you and call the respective function on it. They correctly work on linux by attempting to create a wayland context and falling back to X11 if an error occurs.

## Alternatives

1. [copypasta - Another rust-clipboard fork adding wayland support](https://github.com/alacritty/copypasta)
    The main difference is that copypasta only uses smithay-clipboard, while this library is mainly using
    wl-clipboard-rs since I have not tested the smithay_clipboard code included here at all.  If you need
    or want to use smithay_clipboard I highly recommend you use copypasta.
1. [The original rust-clipboard](https://github.com/aweinstock314/rust-clipboard)

## License

`coach` is dual-licensed under MIT and Apache2.
