use clap::{App, ArgMatches, SubCommand};
use open;

use crate::{
    cli::{arg_branch, GlobalArgs},
    get_branch_name,
    queries::gitlab_get_mr::get_merge_request,
};

use super::CommandResult;

pub const CMD_IDENTIFIER: &str = "open-mr";
const CMD_ABOUT: &str = r#"
Opens the MR of the currently checked out branch in your default browser.
The branch can be overwritten using the --branch flag.
"#;

/// returns the clap definition for this sub-command
pub fn get_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CMD_IDENTIFIER)
        .about(CMD_ABOUT)
        .alias("o")
        .arg(arg_branch())
}

pub async fn run<'a>(args: &ArgMatches<'a>, global_args: &GlobalArgs) -> CommandResult {
    let branch_name = get_branch_name(args)?;
    let mr = get_merge_request(&global_args.token, &global_args.project_path, &branch_name).await?;

    open::that(mr.web_url)?;

    Ok(())
}
