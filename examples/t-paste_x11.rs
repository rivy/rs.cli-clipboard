#[cfg(target_os = "linux")]
use cli_clipboard::ClipboardProvider;

#[cfg(target_os = "linux")]
use cli_clipboard::x11_clipboard::{Clipboard, Primary, X11ClipboardContext};

#[cfg(target_os = "linux")]
fn main() {
    let ctx_x11_primary = X11ClipboardContext::<Primary>::new();
    let ctx_x11_clipboard = X11ClipboardContext::<Clipboard>::new();

    match ctx_x11_clipboard {
        Ok(mut ctx) => {
            let s = ctx.get_contents().unwrap();
            print!("{}", s)
        }
        Err(_) => {
            eprintln!("Err!: x11 (primary) failed")
        }
    }
    match ctx_x11_primary {
        Ok(mut ctx) => {
            let s = ctx.get_contents().unwrap();
            print!("{}", s)
        }
        Err(_) => {
            eprintln!("Err!: x11 (primary) failed")
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {}
