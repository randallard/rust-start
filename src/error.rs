use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // -- Config
    ConfigMissingEnv(&'static str),

    // -- fs (module)
    EmptyFolder,

    // -- Database
    #[from]
    InformixError(informix_rust::errors::InformixError),

    // -- Externals
    #[from]
    Io(std::io::Error),
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Error::ConfigMissingEnv(val) => write!(fmt, "Config Error - Missing Environment Variable: {}", val),
            Error::EmptyFolder => write!(fmt, "Filesystem Error - Empty Folder"),
            Error::InformixError(err) => write!(fmt, "Database Error: {}", err),
            Error::Io(err) => write!(fmt, "IO Error: {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::InformixError(err) => Some(err),
            Error::Io(err) => Some(err),
            _ => None,
        }
    }
}

// endregion: --- Error Boilerplate