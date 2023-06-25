use crate::error::Error;
use handlebars::{Context, Handlebars, Helper, HelperDef, RenderContext, RenderError, ScopedJson};
use rocket::{
    http::CookieJar,
    request::{FromRequest, Outcome},
    Request,
};
use serde::Serialize;
use translitrs::Transliterator;

#[derive(Serialize, Clone, Debug)]
pub enum Script {
    Latin,
    Cyrillic,
}

const SCRIPT_CODE_LATIN: &'static str = "latin";
const SCRIPT_CODE_CYRILLIC: &'static str = "cyrillic";

const SCRIPT_STRING_LATIN: &'static str = "Latin";
const SCRIPT_STRING_CYRILLIC: &'static str = "Cyrillic";

const DEFAULT_SCRIPT: Script = Script::Latin;

impl Script {
    pub fn from_code<'a>(code: &'a str) -> Result<Script, Error> {
        match code {
            SCRIPT_CODE_LATIN => Ok(Script::Latin),
            SCRIPT_CODE_CYRILLIC => Ok(Script::Cyrillic),
            _ => Err(Error::InvalidLanguageCode),
        }
    }

    pub fn from_string<'a>(string: &'a str) -> Result<Script, Error> {
        match string {
            SCRIPT_STRING_LATIN => Ok(Script::Latin),
            SCRIPT_STRING_CYRILLIC => Ok(Script::Cyrillic),
            _ => Err(Error::InvalidLanguageCode),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Script {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let jar = request.guard::<&CookieJar<'_>>().await.unwrap();
        match jar.get("language") {
            Some(language) => match Script::from_code(language.value()) {
                Ok(language) => Outcome::Success(language),
                Err(_) => Outcome::Success(DEFAULT_SCRIPT),
            },
            None => Outcome::Success(DEFAULT_SCRIPT),
        }
    }
}

pub struct ScriptHelper {
    transliterator: translitrs::Transliterator,
}

impl HelperDef for ScriptHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
        let script = Script::from_string(
            h.param(0)
                .ok_or(RenderError::new(
                    "i18n requires the script as the first arg",
                ))?
                .value()
                .as_str()
                .ok_or(RenderError::new("i18n first arg must be a valid string"))?,
        )
        .map_err(|_| RenderError::new("i18n received invalid script code"))?;

        let sentence = h
            .param(1)
            .ok_or(RenderError::new(
                "i18n requires the sentence as the second arg",
            ))?
            .value()
            .as_str()
            .ok_or(RenderError::new("i18n second arg must be a valid string"))?;

        match script {
            Script::Latin => {
                let transliterated = self.transliterator.process(sentence).unwrap();
                Ok(ScopedJson::Derived(transliterated.into()))
            }
            Script::Cyrillic => Ok(ScopedJson::Derived(sentence.into())),
        }
    }
}

impl ScriptHelper {
    pub fn helper() -> Box<dyn HelperDef + Send + Sync + 'static> {
        Box::new(ScriptHelper {
            transliterator: Transliterator::new(
                translitrs::Charset::Cyrillic,
                translitrs::Charset::Latin,
                false,
                false,
                false,
            ),
        })
    }

    pub fn name() -> &'static str {
        "i18n"
    }
}
