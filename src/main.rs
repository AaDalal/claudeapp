// main.rs opens a folder in the Claude desktop app's Code tab via a claude:// deep link.
//
// Usage: claudeapp [folder] [--print]
//   folder   defaults to the current directory
//   --print  print the URL instead of opening it

use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use std::process::Command;

// Encodes like JavaScript's encodeURIComponent: alphanumerics and -_.!~*'() pass through.
// The desktop app decodes the folder value with URLSearchParams, which expects this encoding.
const COMPONENT: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'!')
    .remove(b'~')
    .remove(b'*')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')');

fn main() {
    let mut folder_arg = String::from(".");
    let mut print_only = false;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--print" | "-p" => print_only = true,
            other => folder_arg = other.to_string(),
        }
    }

    // The deep link requires an absolute path to an existing folder.
    let folder = match std::fs::canonicalize(&folder_arg) {
        Ok(p) if p.is_dir() => p,
        Ok(p) => {
            eprintln!("error: {} is not a folder", p.display());
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("error: cannot resolve {folder_arg}: {e}");
            std::process::exit(1);
        }
    };

    let path = folder.to_str().unwrap_or_else(|| {
        eprintln!("error: path is not valid UTF-8");
        std::process::exit(1);
    });
    let url = format!("claude://code/new?folder={}", utf8_percent_encode(path, COMPONENT));

    if print_only {
        println!("{url}");
        return;
    }

    let status = Command::new("open")
        .arg(&url)
        .status()
        .expect("failed to run open");
    if !status.success() {
        eprintln!("error: open failed for {url}");
        std::process::exit(1);
    }
}
