use std::cmp::PartialEq;
use std::string::ToString;

#[derive(
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumDiscriminants,
    strum_macros::IntoStaticStr,
    Debug,
)]
#[repr(u16)]
pub enum CargoMakeError {
    #[strum(
        to_string = "A cycle between different env variables has been detected \
    (E001, see: https://github.com/sagiegurari/cargo-make#e001 for more information). {0}"
    )]
    EnvVarCycle(String) = 100,

    #[strum(to_string = "Detected cycle while resolving alias {0}: {1}")]
    AliasCycle(String, String) = 101,

    #[strum(to_string = "Circular reference found for task: {0:#?}")]
    CircularReference(String) = 102,

    #[strum(to_string = "Unable to run, minimum required version is: {0}")]
    VersionTooOld(String) = 103,

    #[strum(to_string = "Error while executing command, unable to extract exit code.")]
    ExitCodeValidation = 104,

    #[strum(to_string = "Error while executing command, exit code: {0}")]
    ExitCodeError(i32) = 105,

    #[strum(to_string = "Unable to parse internal descriptor: {0}")]
    DescriptorParseFailed(String) = 106,

    #[strum(to_string = "Unable to parse external file: {0:#?}, {1}")]
    ParseFileFailed(String, String) = 107,

    #[strum(to_string = "{0}")]
    Arity(&'static str) = 108,

    #[strum(to_string = "{0}")]
    MethodCallRestriction(&'static str) = 109,

    #[strum(to_string = "Task {0:#?} is {1}")]
    TaskIs(String, &'static str) = 110,

    #[strum(to_string = "{0}")]
    NotFound(String) = 404,

    // ************************
    // * Library level errors *
    // ************************
    #[strum(to_string = "`std::io::Error` error. {error:?}")]
    StdIoError { error: std::io::Error } = 700,

    #[cfg(feature = "diesel")]
    #[strum(to_string = "`diesel::result::Error` error. {error:?}")]
    DieselError { error: diesel::result::Error } = 705,

    #[cfg(feature = "diesel")]
    #[strum(to_string = "`diesel::r2d2::Error` error. {error:?}")]
    DieselR2d2Error { error: diesel::r2d2::Error } = 706,

    #[cfg(feature = "diesel_migrations")]
    #[strum(to_string = "`diesel_migrations::MigrationError` error. {error:?}")]
    DieselMigrationError {
        error: diesel_migrations::MigrationError,
    } = 707,

    #[cfg(feature = "diesel")]
    #[strum(to_string = "`r2d2::Error` error. {error:?}")]
    R2d2Error { error: diesel::r2d2::PoolError } = 708,

    #[strum(to_string = "`std::fmt::Error` error. {error:?}")]
    StdFmtError { error: std::fmt::Error } = 709,

    #[strum(to_string = "{0:?}")]
    ExitCode(std::process::ExitCode) = 710,

    #[strum(to_string = "`toml::de::Error` error. {error:?}")]
    TomlDeError { error: toml::de::Error } = 720,

    #[strum(to_string = "`fsio::error::FsIOError` error. {error:?}")]
    FsIOError { error: fsio::error::FsIOError } = 730,

    #[strum(to_string = "`cliparser::types::ParserError` error. {error:?}")]
    ParserError {
        error: cliparser::types::ParserError,
    } = 731,
}

impl CargoMakeError {
    fn discriminant(&self) -> u16 {
        unsafe { *(self as *const Self as *const u16) }
    }
}

impl From<std::io::Error> for CargoMakeError {
    fn from(error: std::io::Error) -> Self {
        Self::StdIoError { error }
    }
}

impl From<std::fmt::Error> for CargoMakeError {
    fn from(error: std::fmt::Error) -> Self {
        Self::StdFmtError { error }
    }
}

impl From<toml::de::Error> for CargoMakeError {
    fn from(error: toml::de::Error) -> Self {
        Self::TomlDeError { error }
    }
}

impl From<fsio::error::FsIOError> for CargoMakeError {
    fn from(error: fsio::error::FsIOError) -> Self {
        Self::FsIOError { error }
    }
} // ::ParserError

impl From<cliparser::types::ParserError> for CargoMakeError {
    fn from(error: cliparser::types::ParserError) -> Self {
        Self::ParserError { error }
    }
}

impl From<std::process::ExitCode> for CargoMakeError {
    fn from(error: std::process::ExitCode) -> Self {
        Self::ExitCode(error)
    }
}

impl std::process::Termination for CargoMakeError {
    fn report(self) -> std::process::ExitCode {
        if let CargoMakeError::ExitCode(exit_code) = self {
            return exit_code;
        }
        let status_code = self.discriminant();
        if status_code > u8::MAX as u16 {
            eprintln!("exit code {}", status_code);
            std::process::ExitCode::FAILURE
        } else {
            std::process::ExitCode::from(status_code as u8)
        }
    }
}

#[cfg(feature = "diesel")]
impl From<diesel::result::Error> for CargoMakeError {
    fn from(error: diesel::result::Error) -> Self {
        Self::DieselError { error }
    }
}

#[cfg(feature = "diesel")]
impl From<diesel::r2d2::Error> for CargoMakeError {
    fn from(error: diesel::r2d2::Error) -> Self {
        Self::DieselR2d2Error { error }
    }
}

#[cfg(feature = "diesel")]
impl From<diesel::r2d2::PoolError> for CargoMakeError {
    fn from(error: diesel::r2d2::PoolError) -> Self {
        Self::R2d2Error { error }
    }
}

#[cfg(feature = "diesel_migrations")]
impl From<diesel_migrations::MigrationError> for CargoMakeError {
    fn from(error: diesel_migrations::MigrationError) -> Self {
        Self::DieselMigrationError { error }
    }
}

pub enum SuccessOrCargoMakeError<T> {
    Ok(T),
    Err(CargoMakeError),
}

impl<T> From<Result<T, CargoMakeError>> for SuccessOrCargoMakeError<T> {
    fn from(value: Result<T, CargoMakeError>) -> Self {
        match value {
            Ok(val) => SuccessOrCargoMakeError::Ok(val),
            Err(error) => SuccessOrCargoMakeError::Err(error),
        }
    }
}

// Can't use `Result` because
// [E0117] Only traits defined in the current crate can be implemented for arbitrary types
impl<T: std::any::Any> std::process::Termination for SuccessOrCargoMakeError<T> {
    fn report(self) -> std::process::ExitCode {
        const PROCESS_EXIT_CODE: fn(i32) -> std::process::ExitCode = |e: i32| {
            if e > u8::MAX as i32 {
                eprintln!("exit code {}", e);
                std::process::ExitCode::FAILURE
            } else {
                std::process::ExitCode::from(e as u8)
            }
        };

        match self {
            SuccessOrCargoMakeError::Ok(e)
                if std::any::TypeId::of::<T>()
                    == std::any::TypeId::of::<std::process::ExitCode>() =>
            {
                *(&e as &dyn std::any::Any)
                    .downcast_ref::<std::process::ExitCode>()
                    .unwrap()
            }
            SuccessOrCargoMakeError::Ok(_) => std::process::ExitCode::SUCCESS,
            SuccessOrCargoMakeError::Err(err) => match err {
                CargoMakeError::StdIoError { error } if error.raw_os_error().is_some() => {
                    let e = unsafe { error.raw_os_error().unwrap_unchecked() };
                    eprintln!("{}", e.to_string());
                    PROCESS_EXIT_CODE(e)
                }
                CargoMakeError::ExitCode(error) => error,
                _ => {
                    eprintln!("{}", err.to_string());
                    PROCESS_EXIT_CODE(err.discriminant() as i32)
                }
            },
        }
    }
}
