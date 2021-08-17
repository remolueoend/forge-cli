use cli::build_cli;
use commands::CommandResult;
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
