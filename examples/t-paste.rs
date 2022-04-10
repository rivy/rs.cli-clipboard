// spell-checker:ignore (jargon) distro (libs) exitcode OSFILE

use exitcode;
use regex::Regex;

use cli_clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    // ref: [Detect WSL1 vs WSL2](https://github.com/microsoft/WSL/issues/4555) @@ <https://archive.is/uDc9K>
    let is_wsl = std::env::var("IS_WSL").is_ok() || std::env::var("WSL_DISTRO_NAME").is_ok();
    let is_wsl2 = is_wsl && std::env::var("WSL_INTEROP").is_ok();
    let is_wsl1 = !is_wsl2;

    if cfg!(windows) || !is_wsl1 {
        let mut ctx = ClipboardContext::new().unwrap();
        let s = ctx.get_contents().unwrap();
        print!("{}", s);
    } else {
        // is_wsl
        let output = std::process::Command::new("powershell.exe")
            .args([
                "-nonInteractive",
                "-noProfile",
                "-executionPolicy",
                "unrestricted",
                "-command",
                "add-type -AssemblyName System.Windows.Forms; [System.Windows.Forms.Clipboard]::GetText()",
            ])
            .output()
            .unwrap_or_else(|err| {eprintln!("ERR!: `powershell.exe` child process failed ({})", err); std::process::exit(exitcode::OSFILE)});
        let output = String::from_utf8_lossy(&output.stdout);
        // trim trailing newline (removes the unavoidable newline added by `powershell.exe`)
        let rx = Regex::new(r"\r?\n?$").unwrap();
        let s = rx.replace(&output, "");
        print!("{}", s);
    }
}
