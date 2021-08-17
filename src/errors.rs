use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not implemented: {0}.")]
    NotImplemented(String),
    #[error("{message}: {details}")]
    GraphqlError { message: String, details: String },
}
