mod list;

use clap::{ArgMatches, Command};

pub fn builin() -> Vec<Command> {
    vec![list::cli()]
}

type Exec = fn(&ArgMatches);

pub fn builin_exec(cmd: &str) -> Option<Exec> {
    let func = match cmd {
        "list" => list::exec,
        _ => return None,
    };
    Some(func)
}
