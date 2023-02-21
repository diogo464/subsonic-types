#[derive(Debug)]
pub struct Error(Box<dyn std::error::Error>);

impl Error {
    pub fn new<E>(err: E) -> Self
    where
        E: Into<Box<dyn std::error::Error>>,
    {
        Self(err.into())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for Error {}
