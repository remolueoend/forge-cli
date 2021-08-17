use crate::commands;
use clap::{App, AppSettings, Arg, ArgMatches};
use eyre::{Context, Report};

const HELP: &str = r#"
CLI for interacting with Forge tools, such as Gitlab, Github, et al.

Most parameters can be passed as environment variables instead of CLI arguments, allowing you to store these values locally per project in an .envrc/.env file (As long as you're not pushing FORGE_CLI_TOKEN, you're fine).
"#;

const ARG_TOKEN: &str = "token";
const ARG_PROJECT_PATH: &str = "project-path";
const ARG_EDITOR: &str = "editor";
pub const ARG_USE_ORGMODE: &str = "use-org-mode";
pub const ARG_BRANCH: &str = "branch";

/**
 * Returns the global CLI interaface definition.
 * Subcommand definitions are loaded from the respective sub-modules in the commands directory.
 */
pub fn build_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("forge")
        .version("0.1")
        .author("remolueoend")
        .about(HELP)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(commands::edit_merge_request::get_subcommand())
        .subcommand(commands::create_issue::get_subcommand())
        .subcommand(commands::open_mr::get_subcommand())
        .arg(
            Arg::with_name("v")
                .short("v")
                .required(false)
                .multiple(true)
                .help("Sets the level of verbosity")
        )
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

/**
 * Returns the definition of the CLI argument '--use-org-mode', which is used by different sub-commands.
 */
pub fn arg_edit_orgmode<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(ARG_USE_ORGMODE)
		.short("o")
		.takes_value(false)
		.required(false)
		.help("If set, issues and merge requests are translated from markdown to org mode for local editing")
		.env("FORGE_CLI_USE_ORGMODE")
}

pub fn arg_branch<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(ARG_BRANCH)
        .short("-b")
        .takes_value(true)
        .required(false)
        .help("The name of the branch of the merge request to edit. Default is the currently checked out branch")
        .env("FORGE_CLI_BRANCH")
}
pub fn arg_editor<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(ARG_EDITOR)
        .short("e")
        .required(false)
        .takes_value(true)
        .help("The command name of the editor to use. Cannot be an alias! Default is '$EDITOR'")
        .env("FORGE_CLI_EDITOR")
}

/**
 * Contains all global cli options which are independent of the chosen sub-command
 */
pub struct GlobalArgs {
    pub token: String,
    pub project_path: String,
    pub editor_cmd: String,
}
impl<'a> GlobalArgs {
    /// returns a new global options struct based on the parsed CLI arguments
    pub fn from_cli_args(arg_matches: &'a ArgMatches) -> Result<GlobalArgs, Report> {
        Ok(GlobalArgs {
            token: arg_matches.value_of(ARG_TOKEN).unwrap().to_string(),
            project_path: arg_matches.value_of(ARG_PROJECT_PATH).unwrap().to_string(),
            editor_cmd: match arg_matches.value_of(ARG_EDITOR) {
                None => std::env::var("EDITOR").wrap_err("Missing EDITOR environment variable")?,
                Some(arg) => String::from(arg),
            },
        })
    }
}
