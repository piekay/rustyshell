use std::fmt::Error;
use dirs::home_dir;
use rustyline::{DefaultEditor};
use rustyline::error::ReadlineError;
use crate::command_handler::command_handler;
use crate::config_handler::prompt::{read_prompt_statement_from_rsh, replace_placeholders};
mod built_ins;
mod config_handler;
mod command_handler;

fn main() -> Result<(), Error> {
    let mut rl = DefaultEditor::new().expect("Couldn't load Readline Editor");

    if rl.load_history(&home_dir().unwrap().join(".rsh_history")).is_err() {
        println!("No history file found, will create one on exit")
    }

    loop {
        if let Ok(print_statement) = read_prompt_statement_from_rsh() {
            let readline = rl.readline(&*replace_placeholders(print_statement.as_str()));
            match readline {
                Ok(line) => {
                    if line.trim() == "exit" {
                        break;
                    }
                    let tilde_handler = line.replace("~", &*home_dir().unwrap().to_string_lossy());
                    rl.add_history_entry(line.as_str()).expect("Error: Couldn't add to history");
                    command_handler(tilde_handler);
                },
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break
                },
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                    break
                }
            }
        }
    }

    rl.save_history(&home_dir().unwrap().join(".rsh_history")).expect("Couldn't write to history file");
    Ok(())
}