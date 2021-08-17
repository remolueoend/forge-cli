use clap::{App, Arg, ArgMatches, SubCommand};
use cmd_lib::{run_cmd, run_fun};
use eyre::Context;
use log::{debug, info, trace};
use std::io::{Read, Write};
use tempfile::{self, NamedTempFile};

use crate::{
    cli::{arg_edit_orgmode, GlobalArgs},
    queries::gitlab_get_mr::get_merge_request,
    queries::gitlab_update_mr_desc::update_merge_request_desc,
};

use super::CommandResult;

pub const CMD_IDENTIFIER: &str = "edit-merge-request";
const CMD_ABOUT: &str = r#"
Opens a new empty buffer in the system text editor and uploads the content to the remote host as a new issue after the editor is closed.
The format used is similar to git commits:
1. The first line of text is regarded as issue title
2. There must be an empty line between the title and the body.
3. All following lines are uploaded as issue description.

By default, the input is expected to follow the ORG format and is converted to markdown automatically before upload.
"#;

/// returns the clap definition for the edit merge-request sub-command
pub fn get_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CMD_IDENTIFIER)
        .about(CMD_ABOUT)
        .alias("em")
        .arg(
            Arg::with_name("branch")
                .short("-b")
                .takes_value(true)
                .required(false)
                .help("The name of the branch of the merge request to edit. Default is the currently checked out branch")
                .env("FORGE_CLI_BRANCH")
        )
        .arg(arg_edit_orgmode())
}

pub async fn run<'a>(args: &ArgMatches<'a>, global_args: &GlobalArgs) -> CommandResult {
    let current_branch = if let Some(branch) = args.value_of("branch") {
        branch.to_string()
    } else {
        run_fun!(git branch --show-current).wrap_err("failed to run git to fetch current branch")?
    };

    debug!("branch: {}", current_branch);
    debug!("project-path: {}", global_args.project_path);
    trace!("token: {}", global_args.token);

    /*
     * get the merge request details from the API:
     */
    let mr = get_merge_request(
        &global_args.token,
        &global_args.project_path,
        &current_branch,
    )
    .await
    .wrap_err_with(|| {
        format!(
            "Failed to fetch the current merge request for project {}, branch {}",
            global_args.project_path, current_branch
        )
    })?;

    /*
     * open merge request description in editor
     */
    let tmp_file = NamedTempFile::new()
        .wrap_err("Could not create temporary file containing the merge request description")?;

    write!(&tmp_file, "{}", mr.description)
        .wrap_err("Failed to write merge request description to temporary file")?;

    let tmp_file_path = tmp_file.path().to_str().unwrap();
    let editor = &global_args.editor_cmd;
    run_cmd!($editor $tmp_file_path)?;

    let mut new_description = String::new();
    let mut updated_file = tmp_file.reopen()?;
    updated_file
        .read_to_string(&mut new_description)
        .wrap_err("Failed to read the updated merge request description")?;

    debug!(
        "updating merge request with new description: '{}'",
        new_description
    );

    /*
     * upload the edited file content as new merge request description:
     */

    update_merge_request_desc(
        &global_args.token,
        &global_args.project_path,
        &mr.iid,
        &new_description,
    )
    .await?;

    info!("Updated merge request {} successfully.", mr.iid);

    Ok(())
}
