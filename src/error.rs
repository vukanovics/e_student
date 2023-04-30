use log::error;

#[derive(Debug)]
pub enum Error {
    Diesel(diesel::result::Error),
    Bcrypt(bcrypt::BcryptError),
    Rand(rand::Error),
    Hex(hex::FromHexError),
    NotLoggedIn,
}

impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        Self::Diesel(value)
    }
}

impl From<bcrypt::BcryptError> for Error {
    fn from(value: bcrypt::BcryptError) -> Self {
        Self::Bcrypt(value)
    }
}

impl From<rand::Error> for Error {
    fn from(value: rand::Error) -> Self {
        Self::Rand(value)
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
