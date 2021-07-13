use crate::{commands, errors::AppError};
use clap::{App, AppSettings, Arg, ArgMatches};

const HELP: &str = r#"
CLI for interacting with Forge tools, such as Gitlab, Github, et al.

Most parameters can be passed as environment variables instead of CLI arguments, allowing you to store these values locally per project in an .envrc/.env file (As long as you're not pushing FORGE_CLI_TOKEN, you're fine).
"#;

const ARG_TOKEN: &str = "token";
const ARG_PROJECT_PATH: &str = "project-path";

pub fn build_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("forge")
        .version("0.1")
        .author("remolueoend")
        .about(HELP)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(commands::em::get_subcommand())
        .arg(
            Arg::with_name(ARG_TOKEN)
                .short("t")
                .required(true)
                .takes_value(true)
                .help("The authentication token to be sent as Bearer token to the remote forge.")
                .env("FORGE_CLI_TOKEN"),
        )
        .arg(
            Arg::with_name(ARG_PROJECT_PATH)
                .short("p")
                .required(true)
                .takes_value(true)
                .help("The path of the current project, e.g. 'myusername/myproject' or 'mygroup/myproject'")
                .env("FORGE_CLI_PROJECT_PATH"),
        )
}

/// Contains all global cli options which are independent of the chosen sub-command
pub struct GlobalArgs {
    pub token: String,
    pub project_path: String,
}
impl<'a> GlobalArgs {
    /// returns a new global options struct based on the parsed CLI arguments
    pub fn from_cli_args(arg_matches: &'a ArgMatches) -> Result<GlobalArgs, AppError> {
        Ok(GlobalArgs {
            token: arg_matches.value_of(ARG_TOKEN).unwrap().to_string(),
            project_path: arg_matches.value_of(ARG_PROJECT_PATH).unwrap().to_string(),
        })
    }
}
