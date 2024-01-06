use std::env;
use std::iter::Peekable;
use std::path::{Path};
use std::slice::Iter;
use dirs::home_dir;

pub fn execute_cd(mut args: Peekable<Iter<&str>>) {
    let homedir = home_dir().unwrap();
    let homedirtostring = homedir.to_string_lossy();
    let new_dir = args.peek().map_or(&*homedirtostring, |x| *x);
    let root = Path::new(&new_dir);

    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", e);
    }
}
