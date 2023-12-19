use std::borrow::Cow;
use std::borrow::Cow::{Borrowed, Owned};
use std::fmt::Error;
use dirs::home_dir;
use rustyline::{Cmd, CompletionType, Config, EditMode, Editor, KeyEvent};
use rustyline::{Completer, Helper, Hinter, Validator};
use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::HistoryHinter;
use rustyline::validate::MatchingBracketValidator;
use crate::command_handler::command_handler;
use crate::config_handler::prompt::{read_prompt_statement_from_rsh, replace_placeholders};
mod built_ins;
mod config_handler;
mod command_handler;

#[derive(Helper, Completer, Hinter, Validator)]
struct MyHelper {
    #[rustyline(Completer)]
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
    colored_prompt: String,
}

impl Highlighter for MyHelper {
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

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize, forced: bool) -> bool {
        self.highlighter.highlight_char(line, pos, forced)
    }
}

fn main() -> Result<(), Error> {
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::Circular)
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
            let p = format!("{}", replace_placeholders(print_statement.as_str()));
            rl.helper_mut().expect("No helper").colored_prompt = format!("\x1b[1;32m{p}\x1b[0m");
            let readline = rl.readline(&p);
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