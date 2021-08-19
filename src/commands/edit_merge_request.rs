use clap::{App, ArgMatches, SubCommand};
use cmd_lib::run_cmd;
use eyre::{Context, Report};
use log::{debug, info, trace};
use std::fs::File;
use std::io::{Read, Write};
use tempfile::{self, tempdir};

use crate::{
    cli::{arg_branch, arg_edit_orgmode, arg_editor, GlobalArgs, ARG_USE_ORGMODE},
    get_branch_name,
    queries::gitlab_get_mr::get_merge_request,
    queries::gitlab_update_mr_desc::update_merge_request_desc,
};

use super::CommandResult;

pub const CMD_IDENTIFIER: &str = "edit-mr";
const CMD_ABOUT: &str = r#"
Guesses the merge request related to the currently checked out branch (if no branch specified via -b) and then will:
1. download its description into a temporary file,
2. open the system editor to edit the merge request description,
3. update the merge request on the remote host.

When called with the -o flag, the downloaded markdown description is converted to ORG and back to markdown before and after editing it.
"#;

/// returns the clap definition for the edit merge-request sub-command
pub fn get_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CMD_IDENTIFIER)
        .about(CMD_ABOUT)
        .alias("em")
        .arg(arg_branch())
        .arg(arg_edit_orgmode())
        .arg(arg_editor())
}

/**
 * Writes the given description to a temporary file and allows the user to edit it.
 * Returns the edited description. Depending on the given args, it may convert the description
 * before and after editing it.
 */
fn edit_mr_description(
    mr_iid: &String,
    description: &String,
    editor_command: &String,
    convert_to_org: bool,
) -> Result<String, Report> {
    let tmp_dir = tempdir().wrap_err("Could not create temp dir")?;

    // write original description to temp file:
    let orig_desc_file_path = tmp_dir.path().join(format!("{}.md", mr_iid));
    let orig_desc_file = File::create(&orig_desc_file_path)?;
    write!(&orig_desc_file, "{}", description)?;

    let mut org_file_path = orig_desc_file_path.clone();
    org_file_path.set_extension("org");

    // if required, write converted description to new temp file:
    let path_to_edit = if convert_to_org {
        run_cmd!(pandoc -f markdown -t org $orig_desc_file_path -o $org_file_path)
            .wrap_err("Could not convert the MR description to ORG")?;
        org_file_path.clone()
    } else {
        orig_desc_file_path.clone()
    };

    run_cmd!($editor_command $path_to_edit).wrap_err_with(|| {
        format!(
            "Could not open {:?} in editor {}",
            path_to_edit, editor_command
        )
    })?;

    // convert the edited description back to markdown if necessary:
    if convert_to_org {
        run_cmd!(pandoc -f org -t markdown $org_file_path -o $orig_desc_file_path)
            .wrap_err("Could not convert the edited description back to markdown")?;
    };

    let mut updated_file = File::open(&orig_desc_file_path).wrap_err_with(|| {
        format!(
            "Could not open {:?} with new MR description",
            orig_desc_file_path
        )
    })?;
    let mut updated_description = String::new();
    File::read_to_string(&mut updated_file, &mut updated_description)
        .wrap_err("Failed to read the updated merge request description")?;

    // cleanup temp files
    tmp_dir.close()?;

    Ok(updated_description)
}

/**
 * Runs the edit-mr sub-command.
 */
pub async fn run<'a>(args: &ArgMatches<'a>, global_args: &GlobalArgs) -> CommandResult {
    let current_branch = get_branch_name(args)?;
    let convert_to_org = args.is_present(ARG_USE_ORGMODE);

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

    let new_description = edit_mr_description(
        &mr.iid,
        &mr.description,
        &global_args.editor_cmd,
        convert_to_org,
    )?;

    /*
     * upload the edited file content as new merge request description:
     */
    debug!(
        "updating merge request with new description: '{}'",
        new_description
    );

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
