use std::result;
use thiserror::Error;

/// Result type for operations that could result in an `OrkhonError`
pub type Result<T> = result::Result<T, OrkhonError>;

#[derive(Error, Debug)]
pub enum OrkhonError {
    #[error("Orkhon: General Error: {0}")]
    General(String),
    #[error("Orkhon: Model Not Found Error: {0}")]
    ModelNotFound(String),
    #[error("Orkhon: Request Error: {0}")]
    RequestError(String),
    #[error("Orkhon: Request Kind Error: {0}")]
    RequestKindError(String),
    #[error("Orkhon: Acquire GIL Error: {0}")]
    AcquireGILError(String),
    #[error("Orkhon: Python Module Error: {0}")]
    PyModuleError(String),
    #[error("Orkhon: Python Callee Error: {0}")]
    PyCallError(String),
    #[error("Orkhon: Python Callee Error: {0}")]
    OsStringCnvError(String),
    #[error("Orkhon: Model Backend Error: {0}")]
    ModelBackendError(#[from] tract_core::TractError),
    #[error("Orkhon: TF Model Backend Error: {0}")]
    TFModelBackendError(String),
    #[error("Orkhon: IO Error: {0}")]
    IOError(#[from] std::io::Error),
}