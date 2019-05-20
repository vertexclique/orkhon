error_chain! {
    errors {
        OrkhonRequestError(t: String) {
            description("Orkhon Request Error")
            display("Orkhon Request Error: '{}'", t)
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
