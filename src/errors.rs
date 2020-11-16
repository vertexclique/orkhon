use std::result;
use thiserror::Error;

/// Result type for operations that could result in an `OrkhonError`
pub type Result<T> = result::Result<T, OrkhonError>;

#[derive(Error, Debug)]
pub enum OrkhonError {
    #[error("Orkhon: Model Not Found Error: {}")]
    ModelNotFound(String),
    #[error("Orkhon: Request Error: {}")]
    RequestError(String),
    #[error("Orkhon: Request Kind Error: {}")]
    RequestKindError(String),
    #[error("Orkhon: Acquire GIL Error: {}")]
    AcquireGILError(String),
    #[error("Orkhon: Python Module Error: {}")]
    PyModuleError(String),
    #[error("Orkhon: Python Callee Error: {}")]
    PyCallError(String),
    #[error("Orkhon: Python Callee Error: {}")]
    OsStringCnvError(String),
    #[error("Orkhon: Model Backend Error: {}")]
    ModelBackendError(#[from] tract_core::errors::TractError),

}

// error_chain! {
//     errors {
//         OrkhonModelNotFoundError(ex: String) {
//             description("Orkhon Model Not Found Error")
//             display("Orkhon Model Not Found Error: {}", ex)
//         }
//
//         OrkhonRequestError(ex: String) {
//             description("Orkhon Request Error")
//             display("Orkhon Request Error: {}", ex)
//         }
//
//         OrkhonRequestKindError(ex: String) {
//             description("Orkhon Request Kind Error")
//             display("Orkhon Request Kind Error: {}", ex)
//         }
//
//         OrkhonAcquireGILError(ex: String) {
//             description("Orkhon Acquire GIL Error")
//             display("Error occured during acquiring GIL: {}", ex)
//         }
//
//         OrkhonPyModuleError(ex: String) {
//             description("Orkhon Python Module Error")
//             display("Python Module Error: {}", ex)
//         }
//
//         OrkhonPyCallError(ex: String) {
//             description("Orkhon Python Callee Error")
//             display("Python Callee Error: {}", ex)
//         }
//
//         OrkhonOsStringCnvError {
//             description("Os string conversion error. Path is not UTF-8.")
//         }
//     }
//
//     links {
//         OrkhonTractError(tract_core::errors::TractError, tract_core::errors::TractErrorKind) #[cfg(unix)];
//     }
//
//     foreign_links {}
// }
