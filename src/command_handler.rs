use std::process::{Child, Command, Stdio};
use crate::built_ins::cd::execute_cd;

pub(crate) fn command_handler(input:String) {
    // must be peekable so we know when we are on the last command
    let mut commands = input.trim().split(" | ").peekable();
    let mut previous_command = None;

    while let Some(command) = commands.next()  {

        let mut parts = command.trim().split_whitespace();

        let part = parts.clone();
        if part.peekable().peek().is_none() {
            return;
        }

        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => execute_cd(args.peekable()),
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
        // block until the final command has finished
        final_command.wait().expect("Fatal Error. Couldn't wait for command execution");
    }
}