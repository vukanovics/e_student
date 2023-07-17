use log::error;

#[derive(Debug)]
pub enum Error {
    Diesel(diesel::result::Error),
    DatabaseEntryNotFound,
    DatabaseDuplicateEntry,
    Bcrypt(bcrypt::BcryptError),
    Rand(rand::Error),
    HandlebarsRender(handlebars::RenderError),
    HandlebarsTemplate(handlebars::TemplateError),
    Hex(hex::FromHexError),
    LettreSmtp(lettre::transport::smtp::Error),
    NotLoggedIn,
    InvalidLanguageCode,
    InvalidAccountTypeValue,
    NoEnrolDropdownsReceived,
}

impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        match value {
            diesel::result::Error::NotFound => Self::DatabaseEntryNotFound,
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) => Self::DatabaseDuplicateEntry,
            e => Self::Diesel(e),
        }
    }
}

impl From<bcrypt::BcryptError> for Error {
    fn from(value: bcrypt::BcryptError) -> Self {
        Self::Bcrypt(value)
    }
}

impl From<handlebars::RenderError> for Error {
    fn from(value: handlebars::RenderError) -> Self {
        Self::HandlebarsRender(value)
    }
}

impl From<handlebars::TemplateError> for Error {
    fn from(value: handlebars::TemplateError) -> Self {
        Self::HandlebarsTemplate(value)
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

impl From<lettre::transport::smtp::Error> for Error {
    fn from(value: lettre::transport::smtp::Error) -> Self {
        Self::LettreSmtp(value)
    }
}

impl From<Error> for rocket::http::Status {
    fn from(val: Error) -> Self {
        error!("Internal server error: {:?}", val);
        rocket::http::Status::InternalServerError
    }
}
