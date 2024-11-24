use std::error::Error;

use clap::{ArgMatches, Command};
use driver::logger;

fn main() {
    logger::setup();

    if let Err(e) = run() {
        logger::error!("An error occurred: {}", e);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = cli().try_get_matches()?;
    // サブコマンド取得
    let (cmd, subcommand_args) = match args.subcommand() {
        Some((cmd, args)) => (cmd, args),
        _ => {
            cli().print_help()?;
            return Ok(());
        }
    };

    // 実行コマンド取得
    let exec = match builin_exec(cmd) {
        Some(exec) => exec,
        _ => {
            // 発生し得ない
            // TODO: ここを改善する
            cli().print_help()?;
            return Ok(());
        }
    };
    exec(subcommand_args);

    Ok(())
}

fn cli() -> Command {
    Command::new("homeserver-dashboard-cli").subcommands(builin())
}

fn builin() -> Vec<Command> {
    vec![Command::new("list")]
}

fn list_exec(args: &ArgMatches) {
    println!("{:?}", args);
}

type Exec = fn(&ArgMatches);

fn builin_exec(cmd: &str) -> Option<Exec> {
    let func = match cmd {
        "list" => list_exec,
        _ => return None,
    };
    Some(func)
}
