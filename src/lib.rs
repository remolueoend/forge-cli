use clap::ArgMatches;
use cli::{build_cli, ARG_BRANCH};
use cmd_lib::run_fun;
use commands::CommandResult;
use eyre::Context;
use eyre::Report;
use loggerv;
use tokio;

pub mod cli;
pub mod commands;
pub mod errors;
pub mod queries;

/**
 * Main entry point of the app. Parses the command line arguments
 * and passes them to `run_command`, which then will run the sub command
 * based on the given CLI arguments.
 */
#[tokio::main]
pub async fn run() -> CommandResult {
    let cli_args = build_cli().get_matches();

    loggerv::Logger::new()
        .verbosity(cli_args.occurrences_of("v"))
        .level(true)
        .module_path(false)
        .init()
        .unwrap();

    commands::run_command(&cli_args).await
}

/**
 * Returns the name of the current branch, either provided by the given CLI arguments,
 * or falling back to the name of the currently checked out branch.
 */
pub fn get_branch_name<'a>(cli_args: &ArgMatches<'a>) -> Result<String, Report> {
    if let Some(branch) = cli_args.value_of(ARG_BRANCH) {
        Ok(branch.to_string())
    } else {
        run_fun!(git branch --show-current).wrap_err("failed to run git to fetch current branch")
    }
}
