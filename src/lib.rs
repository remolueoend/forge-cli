use cli::build_cli;
use errors::AppError;

pub mod cli;
pub mod commands;
pub mod errors;
pub mod git;
pub mod queries;

pub fn run() -> Result<(), AppError> {
    let cli_args = build_cli().get_matches();
    commands::run_command(&cli_args)
}
