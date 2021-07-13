use clap::{App, ArgMatches, SubCommand};

use crate::{cli::GlobalArgs, errors::AppError, git::get_current_branch_name};

use super::CommandResult;

pub const CMD_IDENTIFIER: &str = "em";
const CMD_ABOUT: &str = r#"
Guesses the merge request related to the currently checked out branch and then will:
1. download its description into a temporary file,
2. open the system editor to edit the merge request description,
3. update the merge request on the remote host.
"#;

/// returns the clap definition for the edit merge-request sub-command
pub fn get_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CMD_IDENTIFIER)
        .about(CMD_ABOUT)
        .alias("ep")
}

pub fn run(args: &ArgMatches, global_args: &GlobalArgs) -> CommandResult {
    let current_branch = get_current_branch_name();
    println!("{}", current_branch.unwrap());
    println!("{}", global_args.project_path);

    Err(AppError::NotImplemented(String::from("command/em/run")))
}
