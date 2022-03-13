/*
Copyright 2016 Avraham Weinstock

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

//! # CLI Clipboard
//!
//! cli-clipboard is a fork of
//! [rust-clipboard](https://github.com/aweinstock314/rust-clipboard) that
//! adds wayland support for terminal and window-less applications via
//! [wl-clipboard-rs](https://github.com/YaLTeR/wl-clipboard-rs). For terminal
//! applications it supports copy and paste for both wayland and X11 linux
//! environments, macOS and windows.
//!
//! Also adds convenience functions for [get_contents](fn.get_contents.html) and
//! [set_contents](fn.set_contents.html).
//!
//! On Linux it will first attempt to setup a Wayland clipboard provider.  If that
//! fails it will then fallback to the X11 clipboard provider.
//!
//! ## Examples
//!
//! Using ClipboardContext to create a clipboard provider:
//!
//! ```
//! use cli_clipboard::{ClipboardContext, ClipboardProvider};
//!
//! let mut ctx = ClipboardContext::new().unwrap();
//! let the_string = "Hello, world!";
//! ctx.set_contents(the_string.to_owned()).unwrap();
//! assert_eq!(ctx.get_contents().unwrap(), the_string);
//! ctx.clear();
//! // clearing the clipboard causes get_contents to return Err on macos and windows
//! if cfg!(any(windows, target_os = "macos")) {
//!    if ctx.get_contents().is_ok() {
//!        panic!("Should be Err");
//!    }
//! } else {
//!    assert_eq!(ctx.get_contents().unwrap(), "");
//! }
//! ```
//!
//! Using the helper functions:
//!
//! ```
//! use cli_clipboard;
//!
//! let the_string = "Hello, world!";
//! cli_clipboard::set_contents(the_string.to_owned()).unwrap();
//! assert_eq!(cli_clipboard::get_contents().unwrap(), the_string);
//! ```

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
extern crate x11_clipboard as x11_clipboard_crate;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod common;
pub use common::ClipboardProvider;

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
pub mod wayland_clipboard;

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
pub mod x11_clipboard;

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
pub mod linux_clipboard;

#[cfg(windows)]
pub mod windows_clipboard;

#[cfg(target_os = "macos")]
pub mod macos_clipboard;

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
pub type ClipboardContext = linux_clipboard::LinuxClipboardContext;

#[cfg(windows)]
pub type ClipboardContext = windows_clipboard::WindowsClipboardContext;

#[cfg(target_os = "macos")]
pub type ClipboardContext = macos_clipboard::MacOSClipboardContext;

/// Get the current clipboard contents
///
/// # Example
/// ```
/// cli_clipboard::set_contents("testing".to_owned()).unwrap();
/// assert_eq!(cli_clipboard::get_contents().unwrap(), "testing");
/// ```
pub fn get_contents() -> Result<String> {
    let mut ctx = ClipboardContext::new()?;
    ctx.get_contents()
}

/// Write a string to the clipboard
///
/// This uses the platform default behavior for setting clipboard contents.
/// Other users of the Wayland or X11 clipboard will only see the contents
/// copied to the clipboard so long as the process copying to the
/// clipboard exists. If you need the contents of the clipboard to
/// remain after your application shuts down, consider using the
/// [set_contents_for_duration](fn.set_contents_for_duration.html) function.
/// MacOS and Windows clipboard contents will stick around after your
/// application exits.
///
/// # Example
/// ```
/// cli_clipboard::set_contents("testing".to_owned()).unwrap();
/// assert_eq!(cli_clipboard::get_contents().unwrap(), "testing");
/// ```
pub fn set_contents(data: String) -> Result<()> {
    let mut ctx = ClipboardContext::new()?;
    ctx.set_contents(data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard() {
        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents("some string".to_owned()).unwrap();
        assert_eq!(ctx.get_contents().unwrap(), "some string");
    }
}
