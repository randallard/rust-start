use derive_more::From;
use tracing::error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // -- fs (module)
    EmptyFolder,

    // -- Externals
    #[from]
    Io(std::io::Error),
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Error::EmptyFolder => {
                error!("{self:?}");
                write!(fmt, "{self:?}")
            }
            _ => write!(fmt, "{self:?}"),
        }
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate