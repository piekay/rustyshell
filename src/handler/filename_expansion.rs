use std::{env, fs};
use dirs::home_dir;

pub fn expand(input: String) -> String {
    return if input.contains("*") || input.contains("~") {
        let expand_star = input.replace("*", &*list_files());
        let expand_tilde = expand_star.replace("~", &*home_dir().unwrap().to_string_lossy());
        expand_tilde
    } else {
        input
    }
}

fn list_files() -> String {
    let mut files = Vec::new();
    if let Ok(current_dir) = env::current_dir() {
        if let Ok(entries) = fs::read_dir(&current_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            if let Some(file_name) = entry.file_name().to_str() {
                                files.push(file_name.to_string());
                            }
                        } else if file_type.is_dir() {
                            if let Some(dir_name) = entry.file_name().to_str() {
                                files.push(dir_name.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    files.join(" ")
}