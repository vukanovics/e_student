#[derive(Debug)]
pub enum Error {
    Diesel(diesel::result::Error),
}

impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        Self::Diesel(value)
    }
}
