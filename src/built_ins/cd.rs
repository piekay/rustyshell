use std::env;
use std::iter::Peekable;
use std::path::{Path};
use std::str::SplitWhitespace;

pub fn execute_cd(args: Peekable<SplitWhitespace>) {
    let new_dir = args.peekable().peek().map_or("~", |x| *x);
    let root = Path::new(&new_dir);
    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", e);
    }
}
