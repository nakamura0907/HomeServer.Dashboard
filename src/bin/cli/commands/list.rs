use clap::{ArgMatches, Command};

pub fn cli() -> Command {
    Command::new("list")
}

pub fn exec(args: &ArgMatches) {
    println!("{:?}", args);
}
