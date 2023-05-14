use log::error;

#[derive(Clone, Debug)]
pub enum Error {
    Diesel,
    DatabaseEntryNotFound,
    Bcrypt,
    Rand,
    Hex(hex::FromHexError),
    NotLoggedIn,
    InvalidLanguageCode,
}

impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        error!("Diesel error: {:?}", value);
        match value {
            diesel::result::Error::NotFound => Self::DatabaseEntryNotFound,
            _ => Self::Diesel,
        }
    }
}

impl From<bcrypt::BcryptError> for Error {
    fn from(_value: bcrypt::BcryptError) -> Self {
        Self::Bcrypt
    }
}

impl From<rand::Error> for Error {
    fn from(_value: rand::Error) -> Self {
        Self::Rand
    }
}

impl From<hex::FromHexError> for Error {
    fn from(value: hex::FromHexError) -> Self {
        Self::Hex(value)
    }
}

impl From<Error> for rocket::http::Status {
    fn from(val: Error) -> Self {
        error!("Internal server error: {:?}", val);
        rocket::http::Status::InternalServerError
    }
}
