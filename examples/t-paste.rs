use cli_clipboard::{ClipboardContext, ClipboardProvider};

// fn trim_newline(s: String) -> &str {
//     let mut len = 0;
//     if s.ends_with('\n') {
//         len += 1;
//         if s.ends_with('\r') {
//             len += 1;
//         }
//     } else if s.ends_with('\r') {
//         len += 1;
//     }
//     &s[0..s.len() - len]
// }

fn main() {
    let is_wsl = std::env::var("IS_WSL").is_ok() || std::env::var("WSL_DISTRO_NAME").is_ok();

    if cfg!(windows) || !is_wsl {
        let mut ctx = ClipboardContext::new().unwrap();
        let s = ctx.get_contents().unwrap();
        print!("{}", s);
    } else {
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
            .expect("Failed to spawn child process");
        let s = String::from_utf8_lossy(&output.stdout);
        print!("{}", s.trim_end_matches(|c| c == '\n' || c == '\r'));
    }
}
