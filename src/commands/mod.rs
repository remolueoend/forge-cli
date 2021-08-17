use clap::ArgMatches;
use eyre::{eyre, Report};

use crate::cli::GlobalArgs;

pub mod create_issue;
pub mod edit_merge_request;
pub mod open_mr;

pub type CommandResult = Result<(), Report>;

/// runs the appropriate command based on the provided process arguments
pub async fn run_command<'a>(cli_args: &ArgMatches<'a>) -> CommandResult {
    let global_args = GlobalArgs::from_cli_args(&cli_args)?;

    match cli_args.subcommand() {
        (edit_merge_request::CMD_IDENTIFIER, Some(cmd_args)) => {
            edit_merge_request::run(cmd_args, &global_args).await
        }
        (create_issue::CMD_IDENTIFIER, Some(cmd_args)) => {
            create_issue::run(cmd_args, &global_args).await
        }
        (open_mr::CMD_IDENTIFIER, Some(cmd_args)) => open_mr::run(cmd_args, &global_args).await,
        ("", _) => Err(eyre!("Missing command. Use --help for more info")),
        // should never be called thanks to `clap`s own validation:
        (cmd, _) => Err(eyre!("Invalid or unknown command: {}", cmd.to_string())),
    }
}
