use std::process::Command;

use crate::errors::AppError;

pub fn get_current_branch_name() -> Result<String, AppError> {
    let output = Command::new("git")
        .args(&["branch", "--show-current"])
        .current_dir(".")
        .output();

    match output {
        // TODO: better error handling in case of parsing failure
        Ok(output) => Ok(String::from_utf8(output.stdout).unwrap().trim().to_string()),
        Err(err) => Err(AppError::ExternalCommand(
            String::from("git branch --show-current"),
            err.to_string(),
        )),
    }
}
