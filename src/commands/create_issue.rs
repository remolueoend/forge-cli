use std::fs::File;

use clap::{App, ArgMatches, SubCommand};
use cmd_lib::run_cmd;
use eyre::Context;
use std::io::Read;
use tempfile::tempdir;

use crate::{
    cli::{arg_edit_orgmode, GlobalArgs, ARG_USE_ORGMODE},
    queries::gitlab_create_issue::create_issue,
};

use super::CommandResult;

pub const CMD_IDENTIFIER: &str = "create-issue";
const CMD_ABOUT: &str = r#"
Opens a new empty buffer in the system text editor and uploads the content to the remote host as a new issue after the editor is closed.
The format used is similar to git commits:
1. The first line of text is regarded as issue title
2. There must be an empty line between the title and the body.
3. All following lines are uploaded as issue description.

When called with the -o flag, the input is expected to follow the ORG format and is automatically converted to markdown before upload.
"#;

/// returns the clap definition for the edit merge-request sub-command
pub fn get_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CMD_IDENTIFIER)
        .about(CMD_ABOUT)
        .alias("ci")
        .arg(arg_edit_orgmode())
}

pub async fn run<'a>(args: &ArgMatches<'a>, global_args: &GlobalArgs) -> CommandResult {
    let use_org = args.is_present(ARG_USE_ORGMODE);
    let tmp_file_ext = if use_org { "org" } else { "md" };
    let tmp_dir = tempdir().wrap_err("Could not create temp dir to store issue")?;
    let tmp_file_path = tmp_dir.path().join(format!("new_issue.{}", tmp_file_ext));
    let editor = &global_args.editor_cmd;

    // we assume the editor creates the file if it does not exist:
    run_cmd!($editor $tmp_file_path)?;

    // convert the ORG description to markdown if necessary:
    let path_to_upload = if use_org {
        let mut markdown_path = tmp_file_path.clone();
        markdown_path.set_extension("md");
        run_cmd!(pandoc -f org -t markdown $tmp_file_path -o $markdown_path)
            .wrap_err("Could not convert the description to markdown")?;
        markdown_path
    } else {
        tmp_file_path
    };

    let mut tmp_file = File::open(&path_to_upload).wrap_err_with(|| {
        format!("Could not open {:?} to read the issue description", {
            &path_to_upload
        })
    })?;

    let mut issue_description = String::new();
    File::read_to_string(&mut tmp_file, &mut issue_description)
        .wrap_err("Failed to read the issue description from file")?;

    // cleanup all created tmp resources:
    tmp_dir.close()?;

    // split the user input by the first empty line to separate issue title and issue description:
    let splitted_desc: Vec<&str> = issue_description.splitn(2, "\n\n").collect();
    let (title, description) = if splitted_desc.len() == 1 {
        (String::from(splitted_desc[0]), None)
    } else {
        (
            String::from(splitted_desc[0]),
            Some(String::from(splitted_desc[1])),
        )
    };

    let created_issue = create_issue(
        &global_args.token,
        &global_args.project_path,
        &title,
        &description,
    )
    .await
    .wrap_err("Failed to create issue")?;

    println!("New issue created at {}", created_issue.web_url);

    Ok(())
}
