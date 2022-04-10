#[cfg(target_os = "linux")]
use std::io::Write;

#[cfg(not(target_os = "linux"))]
use cli_clipboard::ClipboardContext;
use cli_clipboard::ClipboardProvider;

#[cfg(target_os = "linux")]
use cli_clipboard::wayland_clipboard::WaylandClipboardContext;
#[cfg(target_os = "linux")]
use cli_clipboard::x11_clipboard::{Clipboard, Primary, X11ClipboardContext};

fn main() {
    let the_string = "Hello, world!";

    #[cfg(not(target_os = "linux"))]
    {
        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents(the_string.to_owned()).unwrap();
    }

    #[cfg(target_os = "linux")]
    {
        let is_wsl = std::env::var("IS_WSL").is_ok() || std::env::var("WSL_DISTRO_NAME").is_ok();

        // ref: <https://stackoverflow.com/questions/5707990/requested-clipboard-operation-did-not-succeed>
        if !is_wsl {
            {
                let ctx_wayland = WaylandClipboardContext::new();
                match ctx_wayland {
                    Ok(mut ctx) => ctx.set_contents(the_string.to_owned()).unwrap(),
                    Err(_) => {
                        eprintln!("Err!: wayland failed")
                    }
                }
            }
            {
                let ctx_x11_clipboard = X11ClipboardContext::<Clipboard>::new();
                match ctx_x11_clipboard {
                    Ok(mut ctx) => ctx.set_contents(the_string.to_owned()).unwrap(),
                    Err(_) => {
                        eprintln!("Err!: x11 (primary) failed")
                    }
                }
            }
            {
                let ctx_x11_primary = X11ClipboardContext::<Primary>::new();
                match ctx_x11_primary {
                    Ok(mut ctx) => ctx.set_contents(the_string.to_owned()).unwrap(),
                    Err(_) => {
                        eprintln!("Err!: x11 (primary) failed")
                    }
                }
            }
        } else {
            // note: opening the clipboard with WaylandClipboardContext or X11ClipboardContext (as above) may leave an open clipboard leak leading to "Requested Clipboard operation did not succeed." errors
            let mut child = std::process::Command::new("powershell.exe")
            // .args(["-nonInteractive", "-noProfile", "-executionPolicy", "unrestricted", "-command", "$i='';while([Console]::IsInputRedirected -and (($c=[Console]::Read()) -ne -1)){$i+=[Convert]::ToChar($c)}; set-clipboard $i"])
            .args(["-nonInteractive", "-noProfile", "-executionPolicy", "unrestricted", "-command", "$i='';while([Console]::IsInputRedirected -and (($c=[Console]::Read()) -ne -1)){$i+=[Convert]::ToChar($c)}; add-type -AssemblyName System.Windows.Forms; [System.Windows.Forms.Clipboard]::SetText($i)"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn child process");
            let mut child_stdin = child.stdin.take().expect("Failed to write to STDIN");
            std::thread::spawn(move || {
                child_stdin
                    .write_all(the_string.to_owned().as_bytes())
                    .expect("Failed to write to STDIN")
            });
            let output = child.wait_with_output().expect("Failed to read STDOUT");
            println!("output = {:?}", output);
        }
    }
}
