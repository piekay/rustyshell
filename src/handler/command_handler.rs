use std::collections::HashMap;
use std::process::{Child, Command, Stdio};
use shellwords::{split};
use crate::built_ins::cd::execute_cd;
use crate::handler::variable_handler::{get_value, set_vars};

pub(crate) fn command_handler(mut input:String, mut env_vars: HashMap<String, String>) -> HashMap<String, String> {
    for vars in env_vars.keys() {
        input = input.replace(&("$".to_string() + vars), &*get_value(vars.to_string(), env_vars.clone()));
    }
    let mut commands = input.trim().split(" | ").peekable();
    let mut previous_command = None;

    while let Some(command) = commands.next() {
        let environment_vars: Vec<&str> = command
            .split_whitespace()
            .filter(|&word| word.contains('='))
            .collect();

        //Maybe needs to get changed in the future
        let parts: Vec<String> = split(command).unwrap().into_iter()
            .filter(|word| !word.contains('='))
            .collect();

        let  mut parts: Vec<&str> = parts.iter().map(|s| s.as_str()).collect();

        for env_var in environment_vars.iter() {
            let parts: Vec<&str> = env_var.split('=').collect();
            if parts.len() == 2 {
                if !parts[0].contains("$"){
                    let key = parts[0];
                    let value = parts[1];
                    env_vars = set_vars(key.parse().unwrap(), value.parse().unwrap(), env_vars);
                } else { println!("{}", "Command not found: ".to_owned() + &command) }
            }
        }

        if parts.is_empty() {
            return env_vars;
        }

        let command = parts.remove(0);
        let args = parts.clone();

        if command.starts_with("$") {
            println!("{}", "Command not found: ".to_owned() + command);
            break
        }

        match command {
            "cd" => execute_cd(args.iter().peekable()),
            command => {
                let stdin = previous_command
                    .map_or(
                        Stdio::inherit(),
                        |output: Child| Stdio::from(output.stdout.unwrap())
                    );

                let stdout = if commands.peek().is_some() {
                    // there is another command piped behind this one
                    // prepare to send output to the next command
                    Stdio::piped()
                } else {
                    // there are no more built_ins piped behind this one
                    // send output to shell stdout
                    Stdio::inherit()
                };

                let output = Command::new(command)
                    .args(args)
                    .stdin(stdin)
                    .stdout(stdout)
                    .spawn();

                match output {
                    Ok(output) => { previous_command = Some(output); },
                    Err(e) => {
                        previous_command = None;
                        eprintln!("{}", e);
                    },
                };
            }
        }
    }

    if let Some(mut final_command) = previous_command {
        final_command.wait().expect("Fatal Error. Couldn't wait for command execution");
    }
    return env_vars;
}