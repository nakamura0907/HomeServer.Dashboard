use std::error::Error;

mod commands;

use clap::Command;
use commands::{builin, builin_exec};
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
        _ => unreachable!()
    };
    exec(subcommand_args);

    Ok(())
}

fn cli() -> Command {
    Command::new("homeserver-dashboard-cli").subcommands(builin())
}
