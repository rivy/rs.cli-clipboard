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
//! Consider this alpha software.  The tests pass on linux, macOS and windows but
//! it has not yet been manually tested on every platform.
//!

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
extern crate x11_clipboard as x11_clipboard_crate;

use anyhow::{anyhow, Result};
use fork::{fork, Fork};
use std::time::Duration;

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

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
pub fn get_clipboard() -> Result<Box<dyn ClipboardProvider>> {
    match wayland_clipboard::WaylandClipboardContext::new() {
        Ok(context) => Ok(Box::new(context)),
        Err(_) => match x11_clipboard::X11ClipboardContext::<x11_clipboard::Clipboard>::new() {
            Ok(context) => Ok(Box::new(context)),
            Err(err) => Err(err),
        },
    }
}

#[cfg(windows)]
pub fn get_clipboard() -> Result<Box<dyn ClipboardProvider>> {
    windows_clipboard::WindowsClipboardContext::new()
}

#[cfg(target_os = "macos")]
pub fn get_clipboard() -> Result<Box<dyn ClipboardProvider>> {
    macos_clipboard::MacOSClipboardContext::new()
}

/// Get the current clipboard contents
///
/// # Example
/// ```
/// cli_clipboard::set_contents("testing".to_owned()).unwrap();
/// assert_eq!(cli_clipboard::get_contents().unwrap(), "testing");
/// ```
pub fn get_contents() -> Result<String> {
    get_clipboard()?.get_contents()
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
    get_clipboard()?.set_contents(data)?;
    Ok(())
}

/// Write a string to the clipboard for a given duration
///
/// Duration is an Option.  If None is passed in, the clipboard contents
/// will stay set until something else is copied to the clipboard.
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
pub fn set_contents_for_duration(data: String, time: Option<Duration>) -> Result<()> {
    match fork() {
        Ok(Fork::Parent(_)) => Ok(()),
        Ok(Fork::Child) => {
            let mut context = get_clipboard().unwrap();
            context.set_contents(data.clone()).unwrap();
            match time {
                Some(time) => {
                    std::thread::sleep(time);
                    std::process::exit(0);
                }
                None => loop {
                    std::thread::sleep(Duration::from_secs(60));
                    if context.get_contents().unwrap() != data {
                        std::process::exit(0);
                    }
                },
            }
        }
        Err(_e) => Err(anyhow!("Error copying to clipboard")),
    }
}

/// Write a string to the clipboard for a given duration
///
/// Duration is an Option.  If None is passed in, the clipboard contents
/// will stay set until something else is copied to the clipboard.
///
/// # Example
/// ```
/// cli_clipboard::set_contents("testing".to_owned()).unwrap();
/// assert_eq!(cli_clipboard::get_contents().unwrap(), "testing");
/// ```
#[cfg(target_os = "macos")]
pub fn set_contents_for_duration(data: String, time: Option<Duration>) -> Result<()> {
    match fork() {
        Ok(Fork::Parent(_)) => Ok(()),
        Ok(Fork::Child) => {
            let mut context = get_clipboard().unwrap();
            context.set_contents(data.clone()).unwrap();
            match time {
                Some(time) => {
                    std::thread::sleep(time);
                    context.clear().unwrap();
                    std::process::exit(0);
                }
                None => std::process::exit(0),
            }
        }
        Err(_e) => Err(anyhow!("Error copying to clipboard")),
    }
}

/// Write a string to the clipboard for a given duration
///
/// Duration is an Option.  If None is passed in, the clipboard contents
/// will stay set until something else is copied to the clipboard.
///
/// # Example
/// ```
/// cli_clipboard::set_contents("testing".to_owned()).unwrap();
/// assert_eq!(cli_clipboard::get_contents().unwrap(), "testing");
/// ```
#[cfg(windows)]
pub fn set_contents_for_duration(data: String, time: Option<Duration>) -> Result<()> {
    let mut context = get_clipboard()?;
    context.set_contents(data.clone())?;
    match time {
        Some(time) => {
            Command::new("cmd")
                .creation_flags(0x00000008)
                .args(&["sleep", time])
        }
    }
}

#[test]
fn test_clipboard() {
    let mut ctx = get_clipboard().unwrap();
    ctx.set_contents("some string".to_owned()).unwrap();
    assert!(ctx.get_contents().unwrap() == "some string");
}
