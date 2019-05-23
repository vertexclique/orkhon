error_chain! {
    errors {
        OrkhonModelNotFoundError(ex: String) {
            description("Orkhon Model Not Found Error")
            display("Orkhon Model Not Found Error: {}", ex)
        }

        OrkhonRequestError(ex: String) {
            description("Orkhon Request Error")
            display("Orkhon Request Error: {}", ex)
        }

        OrkhonRequestKindError(ex: String) {
            description("Orkhon Request Kind Error")
            display("Orkhon Request Kind Error: {}", ex)
        }

        OrkhonAcquireGILError(ex: String) {
            description("Orkhon Acquire GIL Error")
            display("Error occured during acquiring GIL: {}", ex)
        }

        OrkhonPyModuleError(ex: String) {
            description("Orkhon Python Module Error")
            display("Python Module Error: {}", ex)
        }

        OrkhonPyCallError(ex: String) {
            description("Orkhon Python Callee Error")
            display("Python Callee Error: {}", ex)
        }

        OrkhonOsStringCnvError {
            description("Os string conversion error. Path is not UTF-8.")
        }
    }

    links {
        OrkhonTractError(tract_core::errors::TractError, tract_core::errors::TractErrorKind) #[cfg(unix)];
    }

    foreign_links {}
}
