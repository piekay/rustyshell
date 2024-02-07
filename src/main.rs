use rustyline::completion::{Completer};
use std::borrow::Cow;
use std::borrow::Cow::{Borrowed, Owned};
use std::fmt::Error;
use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use dirs::home_dir;
use rustyline::{Cmd, CompletionType, Config, EditMode, Editor, KeyEvent};
use rustyline::{Helper, Hinter, Validator};
use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::HistoryHinter;
use rustyline::validate::MatchingBracketValidator;
use crate::built_ins::autocomplete::{autocomplete_apps, autocomplete_files};
use crate::built_ins::command_handler::command_handler;
use crate::built_ins::filename_expansion::expand;
use crate::config_handler::prompt::{read_prompt_statement_from_rsh};
use crate::built_ins::variable_handler::{get_vars};

mod built_ins;
mod config_handler;

#[derive(Helper, Hinter, Validator)]
struct MyHelper {
    #[rustyline(Completer)]
    #[allow(dead_code)]
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
    colored_prompt: String,
}

impl Completer for MyHelper {
    type Candidate = String;

    fn complete(&self, line: &str, pos: usize, _ctx: &rustyline::Context<'_>) -> Result<(usize, Vec<String>), ReadlineError> {
        let prefix = &line[..pos];
        let mut app_completions = autocomplete_apps(prefix);
        let file_completions = autocomplete_files(prefix);

        let command_argument = prefix.chars().last().map_or(false, |c| c.is_whitespace());

        if command_argument {
            app_completions.clear();
        }

        let mut completions = app_completions;
        completions.extend(file_completions);
        if prefix.split(" ").last().unwrap().starts_with("/") {
            return Ok((pos, completions))
        }
        Ok((pos - prefix.split(" ").last().unwrap().len(), completions))
    }
}

impl Highlighter for MyHelper {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight_char(&self, line: &str, pos: usize, forced: bool) -> bool {
        self.highlighter.highlight_char(line, pos, forced)
    }
}

fn main() -> Result<(), Error> {
    let running = Arc::new(AtomicBool::new(true));

    let running_clone = Arc::clone(&running);

    let mut env_vars = get_vars().clone();

    ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl+C handler");

    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build();

    let h = MyHelper {
        completer: FilenameCompleter::new(),
        highlighter: MatchingBracketHighlighter::new(),
        hinter: HistoryHinter::new(),
        colored_prompt: "".to_owned(),
        validator: MatchingBracketValidator::new(),
    };

    let mut rl = Editor::with_config(config).expect("Couldn't load Readline Editor");
    rl.set_helper(Some(h));
    rl.bind_sequence(KeyEvent::alt('n'), Cmd::HistorySearchForward);
    rl.bind_sequence(KeyEvent::alt('p'), Cmd::HistorySearchBackward);
    if rl.load_history(&home_dir().unwrap().join(".rsh_history")).is_err() {
        println!("No history file found, will create one on exit")
    }

    loop {
        if let Ok(print_statement) = read_prompt_statement_from_rsh() {
            let p = format!("{}", print_statement.as_str());
            rl.helper_mut().expect("No helper").colored_prompt = format!("\x1b[1;32m{p}\x1b[0m");
            let readline = rl.readline(&p);

            match readline {
                Ok(line) => {
                    if line.trim() == "exit" {
                        break;
                    }
                    rl.add_history_entry(line.as_str()).expect("Error: Couldn't add to history");
                    while running.load(Ordering::SeqCst) {
                        env_vars = command_handler(expand(line), env_vars);
                        break;
                    }
                },
                Err(ReadlineError::Interrupted) => {
                },
                Err(ReadlineError::Eof) => {
                    break
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                    break
                }
            }
        }
        Arc::clone(&running).store(true, Ordering::SeqCst);
    }

    rl.save_history(&home_dir().unwrap().join(".rsh_history")).expect("Couldn't write to history file");
    Ok(())
}