use std::{env, fs, io};
pub(crate) fn read_prompt_statement_from_rsh() -> Result<String, io::Error> {
    if let Some(home_path) = dirs::home_dir() {
        let rsh_path = home_path.join(".rsh");
        match fs::read_to_string(rsh_path) {
            Ok(content) => {
                let prompt_statement = content.lines()
                    .find(|line| line.trim().starts_with("prompt=\"") && line.trim().ends_with("\""))
                    .map(|line| line.trim_matches('"').trim_start_matches("prompt=\"").to_string())
                    .unwrap_or_else(|| "> ".to_string());
                Ok(prompt_statement)
            }
            Err(_) => Ok("> ".to_string()),
        }
    } else {
        Ok("> ".to_string())
    }
}

pub(crate) fn replace_placeholders(prompt: &str) -> String  {
    let username = whoami::username();
    let hostname = whoami::hostname();
    let current_directory = env::current_dir().unwrap().display().to_string();

    let replaced_prompt = prompt
        .replace("$user", &username)
        .replace("$hostname", &hostname)
        .replace("$directory", &current_directory);
    return replaced_prompt;
}