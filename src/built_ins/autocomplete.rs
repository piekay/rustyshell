use std::{env, fs};

pub(crate) fn autocomplete_apps(input: &str) -> Vec<String> {
    let mut suggestions = Vec::new();
    if let Some(executable_dirs) = env::var_os("PATH") {
        let executable_dirs = env::split_paths(&executable_dirs);

        for executable_dir in executable_dirs {
            if let Ok(entries) = fs::read_dir(&executable_dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
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
    return suggestions;
}
pub(crate) fn autocomplete_files(input: &str) -> Vec<String> {
    let mut suggestions = Vec::new();

    let last_arg = if input.split_whitespace().count() <= 1 {""} else { input.split_whitespace().last().expect("Error parsing input") };

    let current_path = env::current_dir().unwrap().display().to_string();

    let index_of_last_slash = if let Some(index_of_last_slash) = last_arg.rfind('/') { index_of_last_slash + 1 } else { 0 };

    let user_specified_path =
        if index_of_last_slash == 1 {
            "/"
        }else if index_of_last_slash != 0 {
            &last_arg[0..index_of_last_slash]
        } else {
            &*current_path
        };

    if let Ok(path) = fs::read_dir(user_specified_path) {
        for entries in path {
            if let Ok(entries) = entries {
                if let Some(file_name) = entries.file_name().to_str() {
                    if !file_name.starts_with(".") || last_arg[index_of_last_slash..].starts_with(".") {
                        if file_name.starts_with(&last_arg[index_of_last_slash..]) || last_arg.is_empty() {
                            let mut new_file_name: String = Default::default();
                            for c in file_name.chars() {
                                if c == ' ' {
                                    new_file_name.push('\\');
                                }
                                new_file_name.push(c);
                            }

                            if entries.file_type().unwrap().is_dir() {
                                new_file_name = new_file_name + "/";
                            }
                            suggestions.push(new_file_name);
                        }
                    }
                }
            }
        }
    }
    return suggestions;
}