use arboard::Clipboard;
use std::io::Write;
use std::path::PathBuf;
use std::{env, fs};
use yaml_rust::YamlLoader;

fn main() {
    let home = env::var("HOME").expect("Could not get HOME environment variable");
    let path: PathBuf = PathBuf::from(format!("{}/.kube/config", home));
    let data = fs::read_to_string(path).expect("could not read file");
    let docs = YamlLoader::load_from_str(&data).expect("yaml format incorrect");
    let doc = &docs[0];

    let token = doc["users"]
        .as_vec()
        .expect("Expected 'users' to be an array")
        .iter()
        .find_map(|user| {
            user["user"]["auth-provider"]["config"]["id-token"]
                .as_str()
                .map(|s| s.to_string())
        })
        .expect("Could not find a user with an 'auth-provider' key");

    match copy_to_clipboard(&token) {
        Err(_) => {
            println!(
                "Failed to copy to clipboard. Here is your token:\n{}",
                token
            );
        }
        Ok(_) => {
            println!("Token copied");
        }
    }
}

fn copy_to_clipboard(token: &str) -> Result<(), ()> {
    if let Ok(mut clipboard) = Clipboard::new() {
        if clipboard.set_text(token.to_owned()).is_ok() {
            println!("Token copied to clipboard!");
            return Ok(());
        }
    }

    #[cfg(target_os = "linux")]
    copy_to_clipboard_wsl(token)
}

#[cfg(target_os = "linux")]
fn copy_to_clipboard_wsl(token: &str) -> Result<(), ()> {
    use std::process::Command;

    if cfg!(target_env = "gnu") {
        let echo = Command::new("echo")
            .arg(token)
            .output()
            .expect("Failed to run echo");

        if echo.status.success() {
            let mut child = Command::new("clip.exe")
                .stdin(std::process::Stdio::piped())
                .spawn()
                .expect("Failed to copy to Windows clipboard via WSL");

            if let Some(mut stdin) = child.stdin.take() {
                stdin
                    .write_all(&echo.stdout)
                    .expect("Failed to write to clipboard");
            }

            return Ok(());
        }
    }
    Err(())
}
