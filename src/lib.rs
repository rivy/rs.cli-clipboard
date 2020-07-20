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
//! [wl-clipboard-rs](https://github.com/YaLTeR/wl-clipboard-rs)
//!
//! Also adds convenience functions for [get_contents](fn.get_contents.html) and
//! [set_contents](fn.set_contents.html). These functions are particularly useful for
//! linux cli applications since they will attempt to use the wayland clipboard and
//! correctly fallback to X11.
//!

#![crate_name = "cli_clipboard"]
#![crate_type = "lib"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
extern crate wl_clipboard_rs;

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
extern crate x11_clipboard as x11_clipboard_crate;

#[cfg(windows)]
extern crate clipboard_win;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;
#[cfg(target_os = "macos")]
extern crate objc_foundation;
#[cfg(target_os = "macos")]
extern crate objc_id;

use anyhow::Result;

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

#[cfg(windows)]
pub mod windows_clipboard;

#[cfg(target_os = "macos")]
pub mod macos_clipboard;

pub mod nop_clipboard;

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
pub type ClipboardContext = x11_clipboard::X11ClipboardContext;
#[cfg(windows)]
pub type ClipboardContext = windows_clipboard::WindowsClipboardContext;
#[cfg(target_os = "macos")]
pub type ClipboardContext = macos_clipboard::MacOSClipboardContext;
#[cfg(target_os = "android")]
pub type ClipboardContext = nop_clipboard::NopClipboardContext; // TODO: implement AndroidClipboardContext (see #52)
#[cfg(not(any(
    unix,
    windows,
    target_os = "macos",
    target_os = "android",
    target_os = "emscripten"
)))]
pub type ClipboardContext = nop_clipboard::NopClipboardContext;

/// Get the current clipboard contents
///
/// # Example
/// ```
/// cli_clipboard::set_contents("testing".to_owned()).unwrap();
/// assert_eq!(cli_clipboard::get_contents().unwrap(), "testing");
/// ```
#[cfg(all(
    unix,
    not(any(
        windows,
        target_os = "macos",
        target_os = "android",
        target_os = "emscripten"
    ))
))]
pub fn get_contents() -> Result<String> {
    match wayland_clipboard::WaylandClipboardContext::new() {
        Ok(mut context) => context.get_contents(),
        Err(_) => {
            let mut context = ClipboardContext::new()?;
            context.get_contents()
        }
    }
}

/// Get the current clipboard contents
///
/// # Example
/// ```
/// cli_clipboard::set_contents("testing".to_owned()).unwrap();
/// assert_eq!(cli_clipboard::get_contents().unwrap(), "testing");
/// ```
#[cfg(any(target_os = "macos", windows))]
pub fn get_contents() -> Result<String> {
    let mut context = ClipboardContext::new()?;
    context.get_contents()
}

/// Write a string to the clipboard
///
/// Other users of the Wayland or X11 clipboard will only see the contents
/// copied to the clipboard so long as the process copying to the
/// clipboard exists. If you need the contents of the clipboard to
/// remain after your application shuts down, consider daemonizing the
/// clipboard components of your application.
///
/// # Example
/// ```
/// cli_clipboard::set_contents("testing".to_owned()).unwrap();
/// assert_eq!(cli_clipboard::get_contents().unwrap(), "testing");
/// ```
#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
pub fn set_contents(data: String) -> Result<()> {
    match wayland_clipboard::WaylandClipboardContext::new() {
        Ok(mut context) => context.set_contents(data),
        Err(_) => {
            let mut context = ClipboardContext::new()?;
            context.set_contents(data)
        }
    }
}

/// Write a string to the clipboard
///
/// Other users of the Wayland or X11 clipboard will only see the contents
/// copied to the clipboard so long as the process copying to the
/// clipboard exists. If you need the contents of the clipboard to
/// remain after your application shuts down, consider daemonizing the
/// clipboard components of your application.
///
/// # Example
/// ```
/// cli_clipboard::set_contents("testing".to_owned()).unwrap();
/// assert_eq!(cli_clipboard::get_contents().unwrap(), "testing");
/// ```
#[cfg(any(target_os = "macos", windows))]
pub fn set_contents(data: String) -> Result<()> {
    let mut context = ClipboardContext::new()?;
    context.set_contents(data)
}

#[test]
fn test_clipboard() {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents("some string".to_owned()).unwrap();
    assert!(ctx.get_contents().unwrap() == "some string");
}
