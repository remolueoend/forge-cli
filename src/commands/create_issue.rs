use clap::{App, ArgMatches, SubCommand};

use crate::cli::{arg_edit_orgmode, GlobalArgs};

use super::CommandResult;

pub const CMD_IDENTIFIER: &str = "create-issue";
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
        .alias("ci")
        .arg(arg_edit_orgmode())
}

pub async fn run<'a>(args: &ArgMatches<'a>, global_args: &GlobalArgs) -> CommandResult {
    Ok(())
}
