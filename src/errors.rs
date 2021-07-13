use std::fmt::Display;

#[derive(Debug)]
pub enum AppError {
    NotImplemented(String),
    CliMissingCommand,
    CliInvalidCommand(String),
    ExternalCommand(String, String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            AppError::NotImplemented(identifer) => {
                write!(f, "Not Implemented: {}", identifer)
            }
            AppError::CliMissingCommand => {
                write!(f, "missing command; use --help for more info.")
            }
            AppError::CliInvalidCommand(cmd) => {
                write!(f, "Invalid or unknown command {}", cmd)
            }
            AppError::ExternalCommand(cmd, err_msg) => {
                write!(f, "Failed to execute external command {}: {}", cmd, err_msg)
            }
        }
    }
}
