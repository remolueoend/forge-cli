use clap::ArgMatches;

use crate::{cli::GlobalArgs, errors::AppError};

pub mod em;

pub type CommandResult = Result<(), AppError>;

/// runs the appropriate command based on the provided process arguments
pub fn run_command(cli_args: &ArgMatches) -> CommandResult {
    let global_args = GlobalArgs::from_cli_args(&cli_args)?;

    match cli_args.subcommand() {
        (em::CMD_IDENTIFIER, Some(cmd_args)) => em::run(cmd_args, &global_args),
        ("", _) => Err(AppError::CliMissingCommand),
        // should never be called thanks to `clap`s own validation:
        (cmd, _) => Err(AppError::CliInvalidCommand(cmd.to_string())),
    }
}
