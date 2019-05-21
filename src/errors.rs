error_chain! {
    errors {
        OrkhonRequestError(ex: String) {
            description("Orkhon Request Error")
            display("Orkhon Request Error: {}", ex)
        }

        OrkhonAcquireGILError(ex: String) {
            description("Orkhon Acquire GIL Error")
            display("Error occured during acquiring GIL: {}", ex)
        }
    }

    links {
        OrkhonTractError(tract_core::errors::TractError, tract_core::errors::TractErrorKind) #[cfg(unix)];
    }

    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error) #[cfg(unix)];
    }
}
