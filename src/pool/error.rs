#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{}", _0)]
    FailedConstruction(#[cause] ConstructionError),
}

pub type Result<T> = ::std::result::Result<T, Error>;



#[derive(Debug, Fail)]
pub enum ConstructionError {
    #[fail(display = "Failed construction, should try again")]
    TryAgain,
    #[fail(display = "Failed construction, should not try again")]
    Failed,
    #[fail(display = "Failed construction because of {}, should not try again", _0)]
    FailedWithError(Box<::std::error::Error + Sync + Send>),
}

pub type ConstructionResult<T> = ::std::result::Result<T, ConstructionError>;

impl From<ConstructionError> for Error {
    fn from(error: ConstructionError) -> Self {
        Error::FailedConstruction(error)
    }
}
