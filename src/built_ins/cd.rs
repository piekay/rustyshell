use std::env;
use std::iter::Peekable;
use std::path::{Path, PathBuf};
use std::str::SplitWhitespace;

pub fn execute_cd(args: Peekable<SplitWhitespace>) {
    let new_dir = args.peekable().peek().map_or("~", |x| *x);
    let expanded_dir = expand_tilde(new_dir);
    let root = Path::new(&expanded_dir);
    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", e);
    }
}
fn expand_tilde(path: &str) -> String {
    if path.starts_with("~/") {
        if let Some(home_path) = dirs::home_dir() {
            let mut expanded_path = PathBuf::new();
            expanded_path.push(home_path);
            expanded_path.push(&path[2..]); // Skip the tilde
            return expanded_path.to_string_lossy().into_owned();
        }
    }else if path.eq("~") {
        if let Some(home_path) = dirs::home_dir() {
            let mut expanded_path = PathBuf::new();
            expanded_path.push(home_path);
            expanded_path.push(&path[1..]); // Skip the tilde
            return expanded_path.to_string_lossy().into_owned();
        }
    }

    path.to_string()
}
