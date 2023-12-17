use std::{env, fs, io};
use std::collections::HashMap;

pub(crate) fn read_prompt_statement_from_rsh() -> Result<String, io::Error> {
    if let Some(home_path) = dirs::home_dir() {
        let rsh_path = home_path.join(".rsh");
        match fs::read_to_string(rsh_path) {
            Ok(content) => {
                let prompt_statement = content.lines()
                    .find(|line| line.trim().starts_with("prompt=\"") && line.trim().ends_with("\""))
                    .map(|line| line.trim_matches('"').trim_start_matches("prompt=\"").to_string())
                    .unwrap_or_else(|| "> ".to_string()); // Default to "> " if not found
                Ok(prompt_statement)
            }
            Err(_) => Ok("> ".to_string()), // Return "> " on file read error
        }
    } else {
        Ok("> ".to_string()) // Return "> " if home directory is not available
    }
}

pub(crate) fn replace_placeholders(prompt: &str) -> String  {
    let mut result = String::with_capacity(prompt.len());
    let mut chars = prompt.chars().peekable();

    //Hardcoded Replacements
    let mut replacements = HashMap::new();
    replacements.insert("$user", whoami::username());
    replacements.insert("$home", dirs::home_dir().map_or_else(|| "/".to_string(), |p| p.to_string_lossy().into_owned()));
    replacements.insert("$directory", env::current_dir().unwrap().display().to_string());
    replacements.insert("$hostname", whoami::hostname());

    while let Some(ch) = chars.next() {
        if ch == '$' {
            if let Some(&next_char) = chars.peek() {
                if next_char.is_alphanumeric() || next_char == '_' {
                    let placeholder: String = std::iter::once(ch)
                        .chain(chars.by_ref().take_while(|&c| c.is_alphanumeric() || c == '_'))
                        .collect();
                    result.push_str(replacements.get(placeholder.as_str()).unwrap_or(&&placeholder));
                } else {
                    result.push(ch);
                }
            } else {
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
    }
    result
}