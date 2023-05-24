use log::error;

#[derive(Clone, Debug)]
pub enum Error {
    Diesel,
    DatabaseEntryNotFound,
    DatabaseDuplicateEntry,
    Bcrypt,
    Rand,
    HandlebarsRender,
    HandlebarsTemplate,
    Hex(hex::FromHexError),
    LettreSmtp,
    NotLoggedIn,
    InvalidLanguageCode,
    InvalidAccountTypeValue,
}

impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        error!("Diesel error: {:?}", value);
        match value {
            diesel::result::Error::NotFound => Self::DatabaseEntryNotFound,
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) => Self::DatabaseDuplicateEntry,
            _ => Self::Diesel,
        }
    }
}

impl From<bcrypt::BcryptError> for Error {
    fn from(value: bcrypt::BcryptError) -> Self {
        error!("Bcrypt error: {:?}", value);
        Self::Bcrypt
    }
}

impl From<handlebars::RenderError> for Error {
    fn from(_value: handlebars::RenderError) -> Self {
        Self::HandlebarsRender
    }
}

impl From<handlebars::TemplateError> for Error {
    fn from(value: handlebars::TemplateError) -> Self {
        error!("Handlebars template error: {:?}", value);
        Self::HandlebarsTemplate
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

impl From<lettre::transport::smtp::Error> for Error {
    fn from(value: lettre::transport::smtp::Error) -> Self {
        error!("Lettre error: {:?}", value);
        Self::LettreSmtp
    }
}

impl From<Error> for rocket::http::Status {
    fn from(val: Error) -> Self {
        error!("Internal server error: {:?}", val);
        rocket::http::Status::InternalServerError
    }
}
