use std::{env, fs};
use std::path::PathBuf;
pub(crate) fn autocomplete_apps(input: &str) -> Vec<String> {
    let mut suggestions = Vec::new();
    if let Some(executable_dirs) = env::var_os("PATH") {
        let executable_dirs = env::split_paths(&executable_dirs);

        for executable_dir in executable_dirs {
            if let Ok(entries) = fs::read_dir(&executable_dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Ok(file_type) = entry.file_type() {
                            if file_type.is_file() {
                                if let Some(file_name) = entry.file_name().to_str() {
                                    if file_name.starts_with(input) {
                                        if !suggestions.contains(&file_name.to_string().clone()) {
                                            suggestions.push(file_name.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        return Vec::new()
    }
    return suggestions;
}
pub(crate) fn autocomplete_files(input: &str) -> Vec<String> {
    let mut suggestions = Vec::new();

    let last_arg = input.split_whitespace().last();

    if let Some(last_arg) = last_arg {
        if last_arg.starts_with("/") && !last_arg.ends_with("/") {
            if let Some(index) = last_arg.rfind('/') {
                let path: PathBuf = last_arg[..index + 1].to_string().into();
                if let Ok(path) = fs::read_dir(path) {
                    for entries in path {
                        if let Ok(entries) = entries {
                            if let Ok(file_type) = entries.file_type() {
                                if file_type.is_dir() || file_type.is_file() {
                                    if let Some(file_name) = entries.file_name().to_str() {
                                        if file_name.starts_with(&last_arg[index + 1..]) {
                                            if let Some(suffix) = file_name.strip_prefix(&last_arg[index + 1..]) {
                                                let slash = suffix.to_owned() + "/";
                                                suggestions.push(slash.to_string());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }else if input.split_whitespace().count() > 0 {
            if let Ok(path) = fs::read_dir(env::current_dir().unwrap().display().to_string()) {
                for entries in path {
                    if let Ok(entries) = entries {
                        if let Ok(file_type) = entries.file_type() {
                            if file_type.is_dir() || file_type.is_file() {
                                if let Some(file_name) = entries.file_name().to_str() {
                                    if file_name.starts_with(last_arg) {
                                        if !file_name.starts_with(".") {
                                            let slash = file_name.to_owned() + "/";
                                            suggestions.push(slash.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }else {
            let path: PathBuf = last_arg.into();
            if let Ok(path) = fs::read_dir(path) {
                for entries in path {
                    if let Ok(entries) = entries {
                        if let Ok(file_type) = entries.file_type() {
                            if file_type.is_dir() || file_type.is_file() {
                                if let Some(file_name) = entries.file_name().to_str() {
                                    if !file_name.starts_with(".") {
                                        let slash = file_name.to_owned() + "/";
                                        suggestions.push(slash.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if input.ends_with(" ") {
        if let Ok(entries) = fs::read_dir(env::current_dir().unwrap().display().to_string()) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if !file_name.starts_with(".") {
                            let slash = file_name.to_owned() + "/";
                            suggestions.push(slash.to_string());
                        }
                    }
                }
            }
        }
    }
    suggestions
}